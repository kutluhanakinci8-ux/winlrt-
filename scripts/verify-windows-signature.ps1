# Authenticode doğrulama — imzalı CI artifact (Faz 3)
param(
    [Parameter(Mandatory = $false)]
    [string[]]$Paths = @(
        "target/release/bundle/nsis/*-setup.exe",
        "target/release/bundle/**/*.exe"
    )
)

$ErrorActionPreference = "Stop"

function Install-LertivaDevTrust {
    param([string]$CerPath)
    if (-not (Test-Path $CerPath)) { return $false }
    Write-Host "Dev codesign güveni: $CerPath → LocalMachine Root + TrustedPublisher"
    Import-Certificate -FilePath $CerPath -CertStoreLocation Cert:\LocalMachine\Root | Out-Null
    Import-Certificate -FilePath $CerPath -CertStoreLocation Cert:\LocalMachine\TrustedPublisher | Out-Null
    return $true
}

$trustDev = $env:WINLRT_TRUST_DEV_CODESIGN -ne "0"
$devCer = Join-Path $PSScriptRoot ".." ".github" "signing" "lertiva-dev-codesign.cer" | Resolve-Path -ErrorAction SilentlyContinue
if ($trustDev -and $devCer) {
    Install-LertivaDevTrust -CerPath $devCer | Out-Null
}

$signtool = "${env:ProgramFiles(x86)}\Windows Kits\10\bin\*\x64\signtool.exe"
$resolved = Get-Item $signtool -ErrorAction SilentlyContinue | Sort-Object FullName -Descending | Select-Object -First 1
if (-not $resolved) {
    Write-Error "signtool.exe bulunamadı (Windows SDK)."
}

$files = @()
foreach ($pattern in $Paths) {
    $files += Get-ChildItem -Path $pattern -ErrorAction SilentlyContinue
}
$files = $files | Where-Object { $_.PSIsContainer -eq $false } | Sort-Object FullName -Unique
if ($files.Count -eq 0) {
    Write-Error "Doğrulanacak .exe/.msi bulunamadı."
}

$fail = 0
foreach ($f in $files) {
    Write-Host "signtool verify /pa $($f.FullName)"
    & $resolved.FullName verify /pa $f.FullName
    if ($LASTEXITCODE -ne 0) { $fail++ }
}
if ($fail -gt 0) {
    Write-Error "$fail dosyada imza doğrulaması başarısız."
}
Write-Host "Tüm imzalar geçerli ($($files.Count) dosya)."
