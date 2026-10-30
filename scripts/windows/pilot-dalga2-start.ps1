# Dalga 2 — beta indirme + kurulum sonrası preflight yönlendirme.
#Usage: powershell -ExecutionPolicy Bypass -File pilot-dalga2-start.ps1
$ErrorActionPreference = 'Stop'

$release = 'https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/winlrt-rar-beta.41064653814'
$preflight = Join-Path $PSScriptRoot 'explorer-qa-preflight.ps1'

Write-Host '=== Lertiva Arc Dalga 2 (Explorer QA) ==='
Write-Host "1) Beta sayfasi aciliyor: $release"
Start-Process $release

Write-Host '2) Kurulum: Lertiva Arc_0.1.0_x64-setup.exe (SmartScreen uyari normal — imza yok)'
Write-Host '3) Kurulum bitince bu klasorde preflight calistirin:'

if (Test-Path $preflight) {
  Write-Host "   powershell -ExecutionPolicy Bypass -File `"$preflight`""
  $run = Read-Host 'Kurulu mu? Preflight simdi calissin mi? (E/H)'
  if ($run -match '^[Ee]') {
    & $preflight
  }
} else {
  Write-Host '   explorer-qa-preflight.ps1 (repo scripts/windows/)'
}

Write-Host ''
Write-Host '4) Checklist: docs/qa/DALGA2_EXPLORER_QA_KOPYA.md'
Write-Host '5) Cursor chat (sablon): docs/qa/PILOT_CHAT_SABLONLARI.md — QA bitti + SHA256 + OS'
