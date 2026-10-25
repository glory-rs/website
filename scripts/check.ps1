param([switch]$Browser)
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
Push-Location $projectRoot
try {
    $env:CARGO_HOME = 'D:/BuildCache/glory-semantic-ui/cargo'
    $env:CARGO_TARGET_DIR = 'D:/BuildCache/glory-website/target'
    $env:CARGO_BUILD_JOBS = '2'
    $env:CARGO_INCREMENTAL = '0'
    $env:TEMP = 'D:/BuildCache/glory-website/tmp'
    $env:TMP = $env:TEMP
    New-Item -ItemType Directory -Force -Path $env:TEMP | Out-Null
    cargo fmt --all -- --check
    if ($LASTEXITCODE -ne 0) { throw 'format check failed' }
    cargo test --locked --all-targets
    if ($LASTEXITCODE -ne 0) { throw 'native tests failed' }
    cargo clippy --locked --all-targets -- -D warnings
    if ($LASTEXITCODE -ne 0) { throw 'native clippy failed' }
    cargo clippy --locked --lib --target wasm32-unknown-unknown -- -D warnings
    if ($LASTEXITCODE -ne 0) { throw 'wasm clippy failed' }
    & "$PSScriptRoot/build.ps1" -Serve -Browser:$Browser
    if ($LASTEXITCODE -ne 0) { throw 'website build failed' }
} finally {
    Pop-Location
}
