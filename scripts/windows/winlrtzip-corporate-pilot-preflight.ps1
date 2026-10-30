# winlrtZIP W18 — kurumsal politika okuma.
$ErrorActionPreference = 'Stop'

$paths = @(
  'HKCU:\Software\winlrt\winlrtZIP',
  'HKLM:\Software\winlrt\winlrtZIP',
  'HKCU:\Software\winlrt\winlrtRAR',
  'HKLM:\Software\winlrt\winlrtRAR'
)

Write-Host '--- winlrtZIP kurumsal politika ---'
$zipFound = $false
foreach ($p in $paths) {
  if (-not (Test-Path $p)) { continue }
  if ($p -match 'winlrtZIP') { $zipFound = $true }
  Write-Host "[$p]"
  $props = Get-ItemProperty -Path $p
  @('PasswordPolicy', 'RequireModernCrypto', 'AuditMirrorPath', 'AuditForwardUrl') | ForEach-Object {
    if ($null -ne $props.$_) { Write-Host "  $_ = $($props.$_)" }
  }
}

if (-not $zipFound) {
  Write-Host '[INFO] winlrtZIP anahtari yok — winlrtzip-corporate-policy-sample.reg veya GPO.'
  exit 2
}

$pp = (Get-ItemProperty -Path 'HKLM:\Software\winlrt\winlrtZIP' -ErrorAction SilentlyContinue).PasswordPolicy
if (-not $pp) {
  $pp = (Get-ItemProperty -Path 'HKCU:\Software\winlrt\winlrtZIP' -ErrorAction SilentlyContinue).PasswordPolicy
}
if ($pp -ne 'corporate') {
  Write-Host '[WARN] PasswordPolicy=corporate beklenir.'
  exit 1
}

Write-Host '[OK] winlrtZIP kurumsal anahtarlar — pilot senaryolara geçin.'
exit 0
