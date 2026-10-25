param(
    [Parameter(Mandatory)][string]$Directory,
    [string]$ProjectName = 'glory-website',
    [string]$WranglerVersion = '4.152.0'
)
$ErrorActionPreference = 'Stop'
if (-not $env:CLOUDFLARE_API_TOKEN -or -not $env:CLOUDFLARE_ACCOUNT_ID) {
    throw 'Set the CLOUDFLARE_API_TOKEN and CLOUDFLARE_ACCOUNT_ID GitHub Actions secrets before deploying.'
}
if ($env:CLOUDFLARE_ACCOUNT_ID -notmatch '^[a-fA-F0-9]{32}$') { throw 'Invalid Cloudflare Account ID' }
if ($ProjectName -notmatch '^[a-z0-9][a-z0-9-]{0,57}[a-z0-9]$') { throw 'Invalid Cloudflare Pages project name' }
foreach ($file in @('index.html','404.html','asset-manifest.json','_headers')) {
    if (-not (Test-Path -LiteralPath (Join-Path $Directory $file) -PathType Leaf)) { throw "Missing deployment asset: $file" }
}
& python "$PSScriptRoot/site-assets.py" retain $Directory https://glory.taidge.org
if ($LASTEXITCODE -ne 0) { throw 'Previous asset retention failed' }
& python "$PSScriptRoot/site-assets.py" validate $Directory
if ($LASTEXITCODE -ne 0) { throw 'Deployment asset validation failed' }
$headers = @{ Authorization = "Bearer $env:CLOUDFLARE_API_TOKEN" }
$endpoint = "https://api.cloudflare.com/client/v4/accounts/$env:CLOUDFLARE_ACCOUNT_ID/pages/projects"
try {
    $response = Invoke-RestMethod -Uri "$endpoint/$ProjectName" -Headers $headers
} catch {
    if ([int]$_.Exception.Response.StatusCode -ne 404) { throw }
    $body = @{ name = $ProjectName; production_branch = 'main' } | ConvertTo-Json
    $response = Invoke-RestMethod -Method Post -Uri $endpoint -Headers $headers -ContentType 'application/json' -Body $body
}
if (-not $response.success) { throw 'Cloudflare Pages project lookup/creation failed' }
if ($response.result.production_branch -ne 'main') { throw 'The Pages production branch must be main' }
$arguments = @('--yes',"wrangler@$WranglerVersion",'pages','deploy',$Directory,"--project-name=$ProjectName",'--branch=main')
if ($env:GITHUB_SHA) { $arguments += "--commit-hash=$env:GITHUB_SHA" }
& npx @arguments
if ($LASTEXITCODE -ne 0) { throw 'Cloudflare Pages deployment failed' }
