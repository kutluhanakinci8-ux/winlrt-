# Lertiva geliştirme Authenticode (self-signed) — gerçek sertifika gelene kadar.
#Requires -Version 5.1
param(
    [string]$OutDir = "",
    [string]$Password = "",
    [string]$Subject = "CN=Lertiva Dev Code Signing, O=Lertiva, C=TR",
    [int]$ValidDays = 825
)

$ErrorActionPreference = "Stop"
$Root = Split-Path (Split-Path $PSScriptRoot -Parent) -Parent
if (-not $OutDir) { $OutDir = Join-Path $Root ".signing-local" }
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
$PublicDir = Join-Path $Root ".github\signing"
New-Item -ItemType Directory -Force -Path $PublicDir | Out-Null

if (-not $Password) {
    $sec = Read-Host "PFX parolası (GitHub TAURI_SIGNING_PRIVATE_KEY_PASSWORD)" -AsSecureString
    $Password = [Runtime.InteropServices.Marshal]::PtrToStringAuto(
        [Runtime.InteropServices.Marshal]::SecureStringToBSTR($sec))
}

$cert = New-SelfSignedCertificate `
    -Type CodeSigningCert `
    -Subject $Subject `
    -KeyAlgorithm RSA `
    -KeyLength 3072 `
    -HashAlgorithm SHA256 `
    -NotAfter (Get-Date).AddDays($ValidDays) `
    -CertStoreLocation "Cert:\CurrentUser\My"

$thumb = $cert.Thumbprint
$pfxPath = Join-Path $OutDir "lertiva-dev-codesign.pfx"
$cerPath = Join-Path $PublicDir "lertiva-dev-codesign.cer"

Export-PfxCertificate -Cert "Cert:\CurrentUser\My\$thumb" -FilePath $pfxPath -Password (
    ConvertTo-SecureString -String $Password -Force -AsPlainText) | Out-Null
Export-Certificate -Cert "Cert:\CurrentUser\My\$thumb" -FilePath $cerPath -Type CERT | Out-Null

Write-Host "OK: PFX (private) $pfxPath"
Write-Host "OK: CER (public)  $cerPath"
Write-Host "Thumbprint: $thumb"
Write-Host ""
Write-Host "GitHub: bash scripts/push-lertiva-dev-signing-secrets.sh"
Write-Host "SmartScreen: bilinmeyen yayıncı uyarısı devam eder; signtool verify /pa geçer."
