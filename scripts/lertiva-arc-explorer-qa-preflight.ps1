# Lertiva Arc Explorer QA (Dalga 2) — Zip script ile aynı matris.
# Kullanım: powershell -ExecutionPolicy Bypass -File lertiva-arc-explorer-qa-preflight.ps1
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
if (-not (Test-Path $root)) { $root = (Get-Location).Path }

Write-Host "==> Lertiva Arc Explorer kayit defteri (HKCU shell)"
$exts = @('.zip', '.wlrt', '.rar', '.7z')
foreach ($ext in $exts) {
  $key = "HKCU:\Software\Classes\SystemFileAssociations$ext\shell"
  if (Test-Path $key) {
    Write-Host "OK: $ext shell anahtarlari mevcut"
  } else {
    Write-Host "UYARI: $ext shell yok (kurulum sonrasi beklenir): $key"
  }
}

$legacy = Join-Path $PSScriptRoot 'windows/explorer-qa-preflight.ps1'
if (Test-Path $legacy) {
  Write-Host "==> Legacy explorer-qa-preflight"
  & $legacy
}

Write-Host "OK: Lertiva Arc Explorer preflight tamam"
