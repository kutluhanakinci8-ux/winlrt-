# NSIS setup.exe — Authenticode (PFX base64 env ile). CI + yerel.
param(
    [string]$Glob = "target/release/bundle/nsis/*-setup.exe"
)

$ErrorActionPreference = "Stop"

$certB64 = $env:WINDOWS_CERTIFICATE
if ([string]::IsNullOrWhiteSpace($certB64)) {
    $certB64 = $env:TAURI_SIGNING_PRIVATE_KEY
}
$pass = $env:WINDOWS_CERTIFICATE_PASSWORD
if ([string]::IsNullOrWhiteSpace($pass)) {
    $pass = $env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD
}

if ([string]::IsNullOrWhiteSpace($certB64)) {
    Write-Warning "WINDOWS_CERTIFICATE / TAURI_SIGNING_PRIVATE_KEY yok — imza atlanıyor."
    exit 0
}

$files = Get-ChildItem -Path $Glob -ErrorAction SilentlyContinue | Where-Object { -not $_.PSIsContainer }
if ($files.Count -eq 0) {
    Write-Error "İmzalanacak setup.exe yok: $Glob"
}

$pfx = Join-Path $env:RUNNER_TEMP "lertiva-codesign.pfx"
if (-not $env:RUNNER_TEMP) { $pfx = Join-Path ([IO.Path]::GetTempPath()) "lertiva-codesign.pfx" }
try {
    $bytes = [Convert]::FromBase64String($certB64.Trim())
    [IO.File]::WriteAllBytes($pfx, $bytes)
} catch {
    Write-Error "PFX base64 çözülemedi: $($_.Exception.Message)"
}

$signtool = "${env:ProgramFiles(x86)}\Windows Kits\10\bin\*\x64\signtool.exe"
$resolved = Get-Item $signtool -ErrorAction SilentlyContinue | Sort-Object FullName -Descending | Select-Object -First 1
if (-not $resolved) {
    Write-Error "signtool.exe bulunamadı (Windows SDK)."
}

$ts = "http://timestamp.digicert.com"
foreach ($f in $files) {
    Write-Host "signtool sign → $($f.FullName)"
    & $resolved.FullName sign /f $pfx /p $pass /tr $ts /td sha256 /fd sha256 $f.FullName
    if ($LASTEXITCODE -ne 0) { Write-Error "signtool sign başarısız: $($f.Name)" }
}
Remove-Item -Force $pfx -ErrorAction SilentlyContinue
Write-Host "OK: $($files.Count) kurulum imzalandı."
