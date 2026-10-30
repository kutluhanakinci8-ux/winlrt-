# Dalga 3 — kurumsal pilot (Explorer signoff sonrası).
#Usage: powershell -ExecutionPolicy Bypass -File pilot-dalga3-start.ps1
$ErrorActionPreference = 'Stop'

$zipUrl = 'https://winlrt.com.tr/downloads/Lertiva Arc-corporate-gpo.zip'
$doc = 'https://github.com/kutluhanakinci8-ux/winlrt-/blob/main/docs/qa/DALGA3_PILOT_OTURUM.md'

Write-Host '=== Lertiva Arc Dalga 3 (Kurumsal pilot) ==='
Write-Host 'Onkosul: Explorer QA signoff PASS (Dalga 2).'
Write-Host ''
Write-Host "1) GPO paketi: $zipUrl"
Write-Host "2) Rehber: $doc"
Write-Host '3) Test PC (yonetici): apply-corporate-sample.ps1 veya ADMX -> PolicyDefinitions'
Write-Host '4) corporate-pilot-preflight.ps1'
Write-Host '5) DALGA3_KURUMSAL_KOPYA.md senaryolari'
Write-Host '6) Chat: kurumsal pilot bitti + Segment + Politika + Onaylayan'

$open = Read-Host 'ZIP indirme sayfasini acayim mi? (E/H)'
if ($open -match '^[Ee]') {
  Start-Process $zipUrl
}

$apply = Join-Path $PSScriptRoot 'apply-corporate-sample.ps1'
if (Test-Path $apply) {
  $do = Read-Host 'Ornek .reg uygulansin mi? (YONETICI gerekir) (E/H)'
  if ($do -match '^[Ee]') {
    & $apply
  }
}
