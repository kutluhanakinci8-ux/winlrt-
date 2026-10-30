# Lertiva Arc — GPO / PolicyDefinitions

## Dosyalar

| Dosya | Hedef (DC veya her istemci) |
|-------|-----------------------------|
| `Lertiva Arc.admx` | `%SystemRoot%\PolicyDefinitions\` |
| `en-US\Lertiva Arc.adml` | `%SystemRoot%\PolicyDefinitions\en-US\` |
| `tr-TR\Lertiva Arc.adml` | `%SystemRoot%\PolicyDefinitions\tr-TR\` |

`gpupdate /force` sonrası **Yönetim Şablonları → Windows Bileşenleri → Lertiva Arc**.

## Hızlı test (OU / tek PC)

`scripts/winlrt-corporate-policy-sample.reg` — `reg import` veya çift tık (HKLM).

Doğrulama: `scripts/windows/corporate-pilot-preflight.ps1`

## Vitrin ZIP

Build: `scripts/package-corporate-gpo-zip.sh` → `apps/web/public/downloads/Lertiva Arc-corporate-gpo.zip`

Lertiva Zip: [README-Lertiva Zip.md](./README-Lertiva Zip.md) · `scripts/package-winlrtzip-corporate-gpo-zip.sh`
