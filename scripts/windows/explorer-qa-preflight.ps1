# Dalga 2 — kurulum sonrası Explorer kayıt defteri / exe doğrulama (Windows pilot).
# Kullanım: powershell -ExecutionPolicy Bypass -File explorer-qa-preflight.ps1
$ErrorActionPreference = 'Stop'

function Test-RegCommand($path) {
  $cmd = (Get-ItemProperty -Path $path -ErrorAction SilentlyContinue).'(default)'
  if (-not $cmd) { return $false, 'komut yok' }
  if ($cmd -notmatch 'Lertiva Arc\.exe') { return $false, "beklenmeyen komut: $cmd" }
  return $true, $cmd
}

$fail = @()
$ok = @()

$pv = (Get-ItemProperty -Path 'HKLM:\Software\winlrt\Lertiva Arc' -ErrorAction SilentlyContinue).ProductVersion
if ($pv) { $ok += "ProductVersion=$pv" } else { $fail += 'HKLM\Software\winlrt\Lertiva Arc\ProductVersion yok' }

$exeCandidates = @(
  "${env:ProgramFiles}\Lertiva Arc\Lertiva Arc.exe",
  "${env:ProgramFiles(x86)}\Lertiva Arc\Lertiva Arc.exe",
  "${env:LocalAppData}\Programs\Lertiva Arc\Lertiva Arc.exe"
)
$exe = $exeCandidates | Where-Object { Test-Path $_ } | Select-Object -First 1
if ($exe) {
  $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $exe).Hash
  $ok += "exe=$exe"
  $ok += "SHA256=$hash"
} else {
  $fail += 'Lertiva Arc.exe bulunamadı (Program Files / LocalAppData)'
}

$archiveKeys = @(
  'Registry::HKEY_CURRENT_USER\Software\Classes\SystemFileAssociations\.zip\shell\WinlrtOpen\command',
  'Registry::HKEY_LOCAL_MACHINE\Software\Classes\SystemFileAssociations\.zip\shell\WinlrtOpen\command'
)
$archiveOk = $false
foreach ($k in $archiveKeys) {
  if (Test-Path $k) {
    $r, $msg = Test-RegCommand $k
    if ($r) { $archiveOk = $true; $ok += "zip WinlrtOpen: $msg"; break }
  }
}
if (-not $archiveOk) { $fail += '.zip WinlrtOpen shell kaydı yok (HKCU/HKLM)' }

$dirKey = 'Registry::HKEY_CURRENT_USER\Software\Classes\Directory\shell\WinlrtCompress\command'
if (-not (Test-Path $dirKey)) {
  $dirKey = 'Registry::HKEY_LOCAL_MACHINE\Software\Classes\Directory\shell\WinlrtCompress\command'
}
if (Test-Path $dirKey) {
  $r, $msg = Test-RegCommand $dirKey
  if ($r) { $ok += "Directory WinlrtCompress: OK" } else { $fail += "Directory shell: $msg" }
} else {
  $fail += 'Directory\shell\WinlrtCompress yok'
}

Write-Host '--- Lertiva Arc Explorer QA preflight ---'
$ok | ForEach-Object { Write-Host "[OK] $_" -ForegroundColor Green }
$fail | ForEach-Object { Write-Host "[FAIL] $_" -ForegroundColor Red }

Write-Host ''
Write-Host 'Sonraki: docs/qa/DALGA2_EXPLORER_QA_KOPYA.md checklist → chat''te QA bitti → agent signoff commit/deploy.'

if ($fail.Count -gt 0) { exit 1 }
exit 0
