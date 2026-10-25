param(
    [switch]$Serve,
    [switch]$Browser,
    [switch]$Release,
    [string]$BuildRoot = 'D:/BuildCache/glory-website',
    [string]$Bindgen = 'wasm-bindgen'
)
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
$savedEnvironment = @{}
$environmentNames = @('CARGO_HOME','CARGO_TARGET_DIR','CARGO_BUILD_JOBS','CARGO_INCREMENTAL','TEMP','TMP','PYTHONPYCACHEPREFIX','GLORY_WEBSITE_BASE','GLORY_WEBSITE_MEDIA')
foreach ($name in $environmentNames) { $savedEnvironment[$name] = [Environment]::GetEnvironmentVariable($name, 'Process') }
function Invoke-WebsiteCargo {
    param([string[]]$Arguments)
    & cargo @Arguments
    if ($LASTEXITCODE -ne 0) { throw "website cargo failed: $($Arguments -join ' ')" }
}
Push-Location $projectRoot
try {
    $BuildRoot = [IO.Path]::GetFullPath($BuildRoot)
    if ($BuildRoot -notmatch '^D:\\BuildCache\\[^\\]+') { throw 'BuildRoot must be inside D:/BuildCache' }
    if (-not $env:CARGO_HOME) { $env:CARGO_HOME = 'D:/BuildCache/glory-semantic-ui/cargo' }
    if ([IO.Path]::GetFullPath($env:CARGO_HOME) -notmatch '^D:\\BuildCache\\[^\\]+') { throw 'CARGO_HOME must be inside D:/BuildCache' }
    $env:CARGO_TARGET_DIR = "$BuildRoot/target"
    $env:CARGO_BUILD_JOBS = '2'
    $env:CARGO_INCREMENTAL = '0'
    $env:TEMP = "$BuildRoot/tmp"
    $env:TMP = $env:TEMP
    $env:PYTHONPYCACHEPREFIX = "$BuildRoot/pycache"
    $dist = "$BuildRoot/dist"
    New-Item -ItemType Directory -Force -Path $BuildRoot,$env:TEMP | Out-Null
    # Generate into an empty directory so removed routes/assets cannot survive a rebuild.
    if (Test-Path -LiteralPath $dist) { Remove-Item -LiteralPath $dist -Recurse -Force }
    New-Item -ItemType Directory -Force -Path $dist | Out-Null
    $mediaDirectory = & python scripts/site-assets.py prepare $dist assets
    if ($LASTEXITCODE -ne 0) { throw 'Media fingerprint failed' }
    $env:GLORY_WEBSITE_MEDIA = "/$mediaDirectory"
    $metadata = & cargo metadata --locked --no-deps --format-version 1 | ConvertFrom-Json
    if ($LASTEXITCODE -ne 0) { throw 'website metadata failed' }
    if ([IO.Path]::GetFullPath($metadata.target_directory) -ne [IO.Path]::GetFullPath($env:CARGO_TARGET_DIR)) { throw "Unexpected website target: $($metadata.target_directory)" }
    Write-Output "Website target verified: $($metadata.target_directory)"
    if ((& $Bindgen --version).Trim() -ne 'wasm-bindgen 0.2.129') { throw 'website requires wasm-bindgen-cli 0.2.129' }
    $profile = if ($Release) { 'release' } else { 'debug' }
    $buildArguments = @('build','--locked','--lib','--target','wasm32-unknown-unknown')
    if ($Release) { $buildArguments += '--release' }
    Invoke-WebsiteCargo -Arguments $buildArguments
    & $Bindgen "$env:CARGO_TARGET_DIR/wasm32-unknown-unknown/$profile/glory_website.wasm" --target web --out-dir $dist
    if ($LASTEXITCODE -ne 0) { throw 'website wasm-bindgen failed' }
    Copy-Item -LiteralPath styles/site.css -Destination "$dist/site.css" -Force
    Copy-Item -LiteralPath ../sapid/styles/sapid.css -Destination "$dist/sapid.css" -Force
    $assetDirectory = & python scripts/site-assets.py bundle $dist
    if ($LASTEXITCODE -ne 0) { throw 'Asset fingerprint failed' }
    Copy-Item -LiteralPath "$projectRoot/deploy/_headers" -Destination "$dist/_headers" -Force
    $generateArguments = @('run','--locked','--bin','generate')
    if ($Release) { $generateArguments += '--release' }
    Invoke-WebsiteCargo -Arguments ($generateArguments + @('--',$dist,$assetDirectory))
    & python scripts/site-assets.py validate $dist
    if ($LASTEXITCODE -ne 0) { throw 'Asset validation failed' }
    $oversized = @(Get-ChildItem -LiteralPath $dist -Recurse -File | Where-Object Length -GT (25 * 1024 * 1024))
    if ($oversized.Count) { throw "Cloudflare Pages assets exceed 25 MiB: $($oversized.Name -join ', ')" }
    Write-Output "Website output: $dist"
    if ($Serve -or $Browser) {
        $listener = Get-NetTCPConnection -LocalAddress 127.0.0.1 -LocalPort 8796 -State Listen -ErrorAction SilentlyContinue
        if (-not $listener) {
            $server = Start-Process -FilePath 'python' -ArgumentList @("$PSScriptRoot/preview.py",'--port','8796','--directory',$dist) -WindowStyle Hidden -PassThru -RedirectStandardOutput "$BuildRoot/server.log" -RedirectStandardError "$BuildRoot/server-errors.log"
            $server.Id | Set-Content -LiteralPath "$BuildRoot/server.pid"
        }
        $ready = $false
        for ($attempt = 0; $attempt -lt 30; $attempt++) {
            try {
                $client = New-Object System.Net.WebClient
                $response = $client.DownloadString('http://127.0.0.1:8796/')
                $client.Dispose()
                if ($response -notlike '*name="glory-render" content="ssr"*') { throw 'port 8796 belongs to another service' }
                $ready = $true
                break
            } catch { Start-Sleep -Milliseconds 100 }
        }
        if (-not $ready) { throw 'website server did not become ready on 8796' }
        Write-Output 'Website URL: http://127.0.0.1:8796/'
    }
    if ($Browser) {
        $env:GLORY_WEBSITE_BASE = 'http://127.0.0.1:8796'
        & node tests/browser.cjs
        if ($LASTEXITCODE -ne 0) { throw 'website browser acceptance failed' }
    }
} finally {
    Pop-Location
    foreach ($name in $environmentNames) { [Environment]::SetEnvironmentVariable($name, $savedEnvironment[$name], 'Process') }
}
