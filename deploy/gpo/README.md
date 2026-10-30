# Lertiva Arc — GPO / PolicyDefinitions

## Dosyalar

| Dosya | Hedef (DC veya her istemci) |
|-------|-----------------------------|
| `lertiva-arc.admx` | `%SystemRoot%\PolicyDefinitions\` |
| `en-US\lertiva-arc.adml` | `%SystemRoot%\PolicyDefinitions\en-US\` |
| `tr-TR\lertiva-arc.adml` | `%SystemRoot%\PolicyDefinitions\tr-TR\` |

`gpupdate /force` sonrası **Yönetim Şablonları → Windows Bileşenleri → Lertiva Arc**.

## Hızlı test (OU / tek PC)

`scripts/lertiva-arc-corporate-policy-sample.reg` — `reg import` veya çift tık (HKLM).

Doğrulama: `scripts/windows/corporate-pilot-preflight.ps1`

## Vitrin ZIP

Build: `scripts/package-corporate-gpo-zip.sh` → `apps/web/public/downloads/lertiva-arc-corporate-gpo.zip`

Lertiva Zip: [README-lertiva-zip.md](./README-lertiva-zip.md) · `scripts/package-lertiva-zip-corporate-gpo-zip.sh`
