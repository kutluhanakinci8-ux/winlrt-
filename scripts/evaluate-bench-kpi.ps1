# Faz 3 KPI özeti — bench JSON (CI / yerel)
param(
    [Parameter(Mandatory = $true)]
    [string]$JsonPath,
    [switch]$Strict
)

if (-not $Strict) {
    $envStrict = $env:WINLRT_KPI_STRICT
    if ($envStrict -eq "1" -or $envStrict -eq "true") {
        $Strict = $true
    }
}

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
if ($start -lt 0 -or $end -le $start) {
    Write-Error "Geçersiz bench JSON dosyası: $JsonPath"
}
$raw = $raw.Substring($start, $end - $start + 1)
$suite = $raw | ConvertFrom-Json
if (-not $suite.kpi) {
    Write-Error "JSON içinde kpi alanı yok (bench --all --json kullanın)."
}

$kpi = $suite.kpi
Write-Host "Profil: $($suite.profile)"
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
    Write-Error "KPI kapısı henüz geçmedi (Strict)."
}
if ($fail) {
    Write-Host "::warning::Faz 3 KPI kapısı henüz tam geçmedi — beta için uyarı."
    exit 0
}
Write-Host "Faz 3 KPI kapısı (bu profil): GEÇTİ"
exit 0
