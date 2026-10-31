# Yerel rakip CLI karsilastirma (Faz 1 — istege bagli, Windows).
param(
  [string]$Fixture = "benchmarks\fixtures\text"
)
$ErrorActionPreference = 'Stop'
Write-Host "Fixture: $Fixture"
if (Get-Command 7z -ErrorAction SilentlyContinue) {
  Measure-Command { 7z a -tzip "$env:TEMP\lertiva-bench.zip" "$Fixture\*" -mx=1 | Out-Null } | ForEach-Object {
    Write-Host "7z zip create: $($_.TotalMilliseconds) ms"
  }
} else {
  Write-Host "7z yok — atlandi"
}
if (Get-Command WinRAR -ErrorAction SilentlyContinue) {
  Write-Host "WinRAR bulundu (manuel bench: WINLRTRAR_PHASE0.md)"
} else {
  Write-Host "WinRAR yok — atlandi"
}
