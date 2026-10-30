# Lertiva Zip W13 — KOBİ e-posta eki preflight (kurulum + hizli ZIP boyut kapisi).
# Kullanim: powershell -ExecutionPolicy Bypass -File lertiva-zip-kobi-pilot-preflight.ps1
$ErrorActionPreference = 'Stop'

$EmailMaxBytes = 10 * 1024 * 1024

$exeCandidates = @(
  "${env:ProgramFiles}\Lertiva Zip\Lertiva Zip.exe",
  "${env:ProgramFiles(x86)}\Lertiva Zip\Lertiva Zip.exe",
  "${env:LocalAppData}\Programs\Lertiva Zip\Lertiva Zip.exe"
)
$exe = $exeCandidates | Where-Object { Test-Path $_ } | Select-Object -First 1

$fail = @()
$ok = @()

if (-not $exe) {
  $fail += 'Lertiva Zip.exe bulunamadi — once NSIS beta kurulumu'
} else {
  $ok += "exe=$exe"
  $tmpRoot = Join-Path $env:TEMP ("lertiva-zip-kobi-{0}" -f [guid]::NewGuid().ToString('N'))
  New-Item -ItemType Directory -Path $tmpRoot | Out-Null
  $folder = Join-Path $tmpRoot 'FaturaPaketi'
  New-Item -ItemType Directory -Path $folder | Out-Null
  Set-Content -Path (Join-Path $folder 'ozet.txt') -Value 'KOBİ preflight' -Encoding UTF8
  1..3 | ForEach-Object {
    $p = Join-Path $folder ("satir{0}.txt" -f $_)
    Set-Content -Path $p -Value ('x' * 65536)
  }
  $proc = Start-Process -FilePath $exe -ArgumentList @('--compress-folder-quick', $folder) -Wait -PassThru -NoNewWindow
  if ($proc.ExitCode -ne 0) {
    $fail += "--compress-folder-quick exit $($proc.ExitCode)"
  } else {
    $zip = Join-Path $tmpRoot 'FaturaPaketi.zip'
    if (-not (Test-Path $zip)) {
      $zip = Join-Path (Split-Path $folder -Parent) 'FaturaPaketi.zip'
    }
    if (-not (Test-Path $zip)) {
      $fail += 'FaturaPaketi.zip olusturulmadi'
    } else {
      $len = (Get-Item -LiteralPath $zip).Length
      $ok += "email-zip=$zip size=$len"
      if ($len -gt $EmailMaxBytes) {
        $fail += "ZIP $len B > e-posta kapisi $EmailMaxBytes B"
      } else {
        $ok += 'email-size-cap=OK'
      }
    }
  }
  Remove-Item -LiteralPath $tmpRoot -Recurse -Force -ErrorAction SilentlyContinue
}

Write-Host '--- Lertiva Zip KOBİ pilot preflight (W13) ---'
$ok | ForEach-Object { Write-Host "[OK] $_" -ForegroundColor Green }
$fail | ForEach-Object { Write-Host "[FAIL] $_" -ForegroundColor Red }

if ($fail.Count -gt 0) { exit 1 }
exit 0
