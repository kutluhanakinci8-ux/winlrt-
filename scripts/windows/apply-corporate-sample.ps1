# Dalga 3 — örnek kurumsal .reg (HKLM). Yönetici PowerShell gerekir.
#Usage: powershell -ExecutionPolicy Bypass -File apply-corporate-sample.ps1
$ErrorActionPreference = 'Stop'

$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole(
  [Security.Principal.WindowsBuiltInRole]::Administrator
)
if (-not $isAdmin) {
  Write-Error 'Yönetici olarak çalıştırın (HKLM yazımı).'
}

$regFile = Join-Path $PSScriptRoot '..\winlrt-corporate-policy-sample.reg'
$regFile = (Resolve-Path $regFile).Path
if (-not (Test-Path $regFile)) {
  Write-Error "Bulunamadı: $regFile — repodan scripts/ altına kopyalayın veya ZIP indirin."
}

Write-Host "Import: $regFile"
reg import $regFile | Out-Host

& (Join-Path $PSScriptRoot 'corporate-pilot-preflight.ps1')
