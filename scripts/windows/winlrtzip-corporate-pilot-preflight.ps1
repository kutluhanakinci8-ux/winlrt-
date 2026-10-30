# Lertiva Zip W18 — kurumsal politika okuma.
$ErrorActionPreference = 'Stop'

$paths = @(
  'HKCU:\Software\winlrt\Lertiva Zip',
  'HKLM:\Software\winlrt\Lertiva Zip',
  'HKCU:\Software\winlrt\Lertiva Arc',
  'HKLM:\Software\winlrt\Lertiva Arc'
)

Write-Host '--- Lertiva Zip kurumsal politika ---'
$zipFound = $false
foreach ($p in $paths) {
  if (-not (Test-Path $p)) { continue }
  if ($p -match 'Lertiva Zip') { $zipFound = $true }
  Write-Host "[$p]"
  $props = Get-ItemProperty -Path $p
  @('PasswordPolicy', 'RequireModernCrypto', 'AuditMirrorPath', 'AuditForwardUrl') | ForEach-Object {
    if ($null -ne $props.$_) { Write-Host "  $_ = $($props.$_)" }
  }
}

if (-not $zipFound) {
  Write-Host '[INFO] Lertiva Zip anahtari yok — winlrtzip-corporate-policy-sample.reg veya GPO.'
  exit 2
}

$pp = (Get-ItemProperty -Path 'HKLM:\Software\winlrt\Lertiva Zip' -ErrorAction SilentlyContinue).PasswordPolicy
if (-not $pp) {
  $pp = (Get-ItemProperty -Path 'HKCU:\Software\winlrt\Lertiva Zip' -ErrorAction SilentlyContinue).PasswordPolicy
}
if ($pp -ne 'corporate') {
  Write-Host '[WARN] PasswordPolicy=corporate beklenir.'
  exit 1
}

Write-Host '[OK] Lertiva Zip kurumsal anahtarlar — pilot senaryolara geçin.'
exit 0
