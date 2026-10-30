# Lertiva Zip — GPO / PolicyDefinitions (W18)

| Dosya | Hedef |
|-------|--------|
| `lertiva-zip.admx` | `%SystemRoot%\PolicyDefinitions\` |
| `en-US\lertiva-zip.adml` | `%SystemRoot%\PolicyDefinitions\en-US\` |
| `tr-TR\lertiva-zip.adml` | `%SystemRoot%\PolicyDefinitions\tr-TR\` |

`gpupdate /force` → **Yönetim Şablonları → Windows Bileşenleri → Lertiva Zip**.

Örnek `.reg`: `scripts/lertiva-zip-corporate-policy-sample.reg`  
Doğrulama: `scripts/windows/lertiva-zip-corporate-pilot-preflight.ps1`

Vitrin ZIP: `scripts/package-lertiva-zip-corporate-gpo-zip.sh`
