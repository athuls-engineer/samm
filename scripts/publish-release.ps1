param(
    [string]$Version = "v0.2.0",
    [string]$Title = "SAMM Release $Version",
    [string]$Notes = "Features 636 curated mods with 622 1-click installable packages, auto-duplicate detection and resolution engine, robust single-instance startup, crash recovery, and custom themes."
)

$ErrorActionPreference = "Stop"

Write-Host "=== SAMM Release Publisher ===" -ForegroundColor Cyan
Write-Host "Target Version: $Version"

# 1. Ensure build binary exists
$binPath = "SAMM.exe"
if (-not (Test-Path $binPath)) {
    Write-Host "Compiling release binary first..." -ForegroundColor Yellow
    npm run build
    Push-Location "src-tauri"
    cargo build --release
    Pop-Location
    Copy-Item "src-tauri\target\release\samm.exe" -Destination "SAMM.exe" -Force
}

# 2. Package zip archive
$zipName = "SAMM-Windows-x64-$Version.zip"
Write-Host "Packaging release zip: $zipName" -ForegroundColor Cyan
Compress-Archive -Path "SAMM.exe" -DestinationPath $zipName -Force

# 3. Create Git Tag
Write-Host "Creating git tag $Version..." -ForegroundColor Cyan
git tag -f -a $Version -m "$Title"

# 4. Push branch and tag to GitHub
Write-Host "Pushing branch and tag to origin..." -ForegroundColor Cyan
try {
    git push origin main --tags --force
    Write-Host "Successfully pushed to GitHub! GitHub Actions will automatically publish the release." -ForegroundColor Green
} catch {
    Write-Host "Note: git push requires your GitHub login credentials or personal access token." -ForegroundColor Yellow
}

# 5. Check if gh CLI is authenticated to create immediate release
$ghAuth = gh auth status 2>&1
if ($ghAuth -match "Logged in to") {
    Write-Host "GitHub CLI authenticated. Publishing release directly..." -ForegroundColor Green
    gh release create $Version "SAMM.exe" $zipName --title "$Title" --notes "$Notes"
    Write-Host "GitHub Release successfully created!" -ForegroundColor Green
} else {
    Write-Host "To publish directly from CLI, run: gh auth login" -ForegroundColor Gray
}

Write-Host "=== Release preparation complete ===" -ForegroundColor Cyan
