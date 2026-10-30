# Lertiva Zip ürün KPI — bench JSON içindeki zip_kpi alanı
param(
    [Parameter(Mandatory = $true)]
    [string]$JsonPath,
    [switch]$Strict,
    [switch]$ReportOnly
)

$ErrorActionPreference = "Stop"
if (-not (Test-Path -LiteralPath $JsonPath)) {
    Write-Error "Dosya yok: $JsonPath"
}

$raw = [System.IO.File]::ReadAllText($JsonPath).Trim()
if ($raw.StartsWith([char]0xFEFF)) {
    $raw = $raw.Substring(1)
}
$start = $raw.IndexOf('{')
$end = $raw.LastIndexOf('}')
$raw = $raw.Substring($start, $end - $start + 1)
$suite = $raw | ConvertFrom-Json
if (-not $suite.zip_kpi) {
    Write-Error "JSON içinde zip_kpi yok (bench --all --json --zip-kpi kullanın)."
}

$kpi = $suite.zip_kpi
Write-Host "Lertiva Zip profil: $($suite.profile)"
Write-Host "Platform: $($suite.platform)"
Write-Host "Sıkıştırma kapısı: $($kpi.compression_gate) ($($kpi.compression_pass_count)/$($kpi.compression_total))"
if ($null -ne $kpi.speed_gate) {
    Write-Host "Hız kapısı (Fast): $($kpi.speed_gate) ($($kpi.speed_pass_count)/$($kpi.speed_total))"
} else {
    Write-Host "Hız kapısı: (bu profilde ölçülmedi)"
}

foreach ($row in $kpi.rows) {
    $speed = if ($null -ne $row.speed_ok) { ", hız=$($row.speed_ok)" } else { "" }
    Write-Host "  - $($row.fixture): sıkıştırma=$($row.compression_ok)$speed"
}

$fail = $false
if (-not $kpi.compression_gate) { $fail = $true }
if ($kpi.speed_gate -eq $false) { $fail = $true }

if ($Strict -and $fail) {
    Write-Error "Lertiva Zip KPI kapısı henüz geçmedi (Strict)."
}
if ($fail) {
    if ($ReportOnly) {
        Write-Host "::notice::Lertiva Zip KPI izleme — kapı henüz tam değil (ReportOnly)."
    } else {
        Write-Host "::warning::Lertiva Zip KPI kapısı henüz tam geçmedi."
    }
    exit 0
}
Write-Host "Lertiva Zip KPI (bu profil): GEÇTİ"
exit 0
