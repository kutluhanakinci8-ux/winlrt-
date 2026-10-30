# Dalga 3 — kurumsal politika kayıt defteri (Explorer signoff sonrası).
# .reg import etmeden önce/sonra karşılaştırma için salt okuma.
$ErrorActionPreference = 'Stop'

$paths = @(
  'HKCU:\Software\winlrt\winlrtRAR',
  'HKLM:\Software\winlrt\winlrtRAR'
)

Write-Host '--- winlrtRAR kurumsal politika (okuma) ---'
$found = $false
foreach ($p in $paths) {
  if (-not (Test-Path $p)) { continue }
  $found = $true
  $props = Get-ItemProperty -Path $p
  Write-Host "[$p]"
  @('PasswordPolicy', 'RequireModernCrypto', 'AuditMirrorPath', 'AuditForwardUrl') | ForEach-Object {
    if ($null -ne $props.$_) { Write-Host "  $_ = $($props.$_)" }
  }
}

if (-not $found) {
  Write-Host '[INFO] Henüz politika yok — scripts/winlrt-corporate-policy-sample.reg veya GPO uygulayın.'
  exit 2
}

$pp = (Get-ItemProperty -Path 'HKLM:\Software\winlrt\winlrtRAR' -ErrorAction SilentlyContinue).PasswordPolicy
if ($pp -ne 'corporate') {
  Write-Host '[WARN] PasswordPolicy=corporate beklenir (HKLM veya HKCU).'
  exit 1
}

Write-Host '[OK] Kurumsal anahtarlar okundu — DALGA3 senaryolarına geçin.'
exit 0
