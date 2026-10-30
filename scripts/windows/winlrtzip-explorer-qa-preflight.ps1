# winlrtZIP W12 — kurulum sonrasi Explorer kayit defteri / exe (Windows pilot).
# Kullanim: powershell -ExecutionPolicy Bypass -File winlrtzip-explorer-qa-preflight.ps1
$ErrorActionPreference = 'Stop'

function Test-RegCommand($path, [string]$exeName) {
  $cmd = (Get-ItemProperty -Path $path -ErrorAction SilentlyContinue).'(default)'
  if (-not $cmd) { return $false, 'komut yok' }
  if ($cmd -notmatch [regex]::Escape($exeName)) { return $false, "beklenmeyen komut: $cmd" }
  return $true, $cmd
}

$fail = @()
$ok = @()

$pv = (Get-ItemProperty -Path 'HKLM:\Software\winlrt\winlrtZIP' -ErrorAction SilentlyContinue).ProductVersion
if ($pv) { $ok += "ProductVersion=$pv" } else { $fail += 'HKLM\Software\winlrt\winlrtZIP\ProductVersion yok' }

$exeCandidates = @(
  "${env:ProgramFiles}\winlrtZIP\winlrtZIP.exe",
  "${env:ProgramFiles(x86)}\winlrtZIP\winlrtZIP.exe",
  "${env:LocalAppData}\Programs\winlrtZIP\winlrtZIP.exe"
)
$exe = $exeCandidates | Where-Object { Test-Path $_ } | Select-Object -First 1
if ($exe) {
  $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $exe).Hash
  $ok += "exe=$exe"
  $ok += "SHA256=$hash"
} else {
  $fail += 'winlrtZIP.exe bulunamadi (Program Files / LocalAppData)'
}

$zipOpenKeys = @(
  'Registry::HKEY_CURRENT_USER\Software\Classes\SystemFileAssociations\.zip\shell\WinlrtZipOpen\command',
  'Registry::HKEY_LOCAL_MACHINE\Software\Classes\SystemFileAssociations\.zip\shell\WinlrtZipOpen\command'
)
$zipOpenOk = $false
foreach ($k in $zipOpenKeys) {
  if (Test-Path $k) {
    $r, $msg = Test-RegCommand $k 'winlrtZIP.exe'
    if ($r) { $zipOpenOk = $true; $ok += "WinlrtZipOpen: $msg"; break }
  }
}
if (-not $zipOpenOk) { $fail += '.zip WinlrtZipOpen shell kaydi yok (HKCU/HKLM)' }

$zipExtractKeys = @(
  'Registry::HKEY_CURRENT_USER\Software\Classes\SystemFileAssociations\.zip\shell\WinlrtZipExtractHere\command',
  'Registry::HKEY_LOCAL_MACHINE\Software\Classes\SystemFileAssociations\.zip\shell\WinlrtZipExtractHere\command'
)
$zipExtractOk = $false
foreach ($k in $zipExtractKeys) {
  if (Test-Path $k) {
    $r, $msg = Test-RegCommand $k 'winlrtZIP.exe'
    if ($r) { $zipExtractOk = $true; $ok += "WinlrtZipExtractHere: OK"; break }
  }
}
if (-not $zipExtractOk) { $fail += '.zip WinlrtZipExtractHere shell kaydi yok' }

$dirKey = 'Registry::HKEY_CURRENT_USER\Software\Classes\Directory\shell\WinlrtZipCompress\command'
if (-not (Test-Path $dirKey)) {
  $dirKey = 'Registry::HKEY_LOCAL_MACHINE\Software\Classes\Directory\shell\WinlrtZipCompress\command'
}
if (Test-Path $dirKey) {
  $r, $msg = Test-RegCommand $dirKey 'winlrtZIP.exe'
  if ($r) { $ok += "Directory WinlrtZipCompress: OK" } else { $fail += "Directory shell: $msg" }
} else {
  $fail += 'Directory\shell\WinlrtZipCompress yok'
}

Write-Host '--- winlrtZIP Explorer QA preflight (W12) ---'
$ok | ForEach-Object { Write-Host "[OK] $_" -ForegroundColor Green }
$fail | ForEach-Object { Write-Host "[FAIL] $_" -ForegroundColor Red }

Write-Host ''
Write-Host 'Sonraki: docs/qa/WINLRTZIP_EXPLORER_QA_KOPYA.md (I1/I2) -> chat''te QA bitti -> agent signoff.'

if ($fail.Count -gt 0) { exit 1 }
exit 0
