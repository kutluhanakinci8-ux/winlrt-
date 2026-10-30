# Lertiva Zip W13 — KOBİ e-posta eki pilot baslangic.
# Usage: powershell -ExecutionPolicy Bypass -File pilot-winlrtzip-kobi-start.ps1
$ErrorActionPreference = 'Stop'

$release = 'https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/winlrt-zip-beta.1'
$preflight = Join-Path $PSScriptRoot 'winlrtzip-kobi-pilot-preflight.ps1'
$explorerPreflight = Join-Path $PSScriptRoot 'winlrtzip-explorer-qa-preflight.ps1'

Write-Host '=== Lertiva Zip W13 — KOBİ pilot (e-posta eki) ==='
Write-Host "1) Beta: $release"
Start-Process $release

Write-Host '2) Kurulum: Lertiva Zip_0.1.0_x64-setup.exe (imzasiz beta — SmartScreen normal)'
Write-Host '3) Kurulum sonrasi preflight (e-posta profili ZIP <= 10 MB):'

if (Test-Path $preflight) {
  Write-Host "   powershell -ExecutionPolicy Bypass -File `"$preflight`""
  $run = Read-Host 'Kurulu mu? KOBİ preflight simdi calissin mi? (E/H)'
  if ($run -match '^[Ee]') {
    & $preflight
  }
} else {
  Write-Host '   winlrtzip-kobi-pilot-preflight.ps1 (repo scripts/windows/)'
}

Write-Host ''
Write-Host '4) Checklist: docs/qa/WINLRTZIP_KOBI_PILOT_KOPYA.md'
Write-Host '5) Explorer I1/I2 henuz yapilmadiysa: winlrtzip-explorer-qa-preflight.ps1'
if (Test-Path $explorerPreflight) {
  Write-Host "   `"$explorerPreflight`""
}
Write-Host '6) Chat: Lertiva Zip KOBİ pilot bitti + OS + SHA256 → agent signoff'
