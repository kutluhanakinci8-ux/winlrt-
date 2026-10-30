# winlrtRAR — GPO / PolicyDefinitions

## Dosyalar

| Dosya | Hedef (DC veya her istemci) |
|-------|-----------------------------|
| `winlrtRAR.admx` | `%SystemRoot%\PolicyDefinitions\` |
| `en-US\winlrtRAR.adml` | `%SystemRoot%\PolicyDefinitions\en-US\` |
| `tr-TR\winlrtRAR.adml` | `%SystemRoot%\PolicyDefinitions\tr-TR\` |

`gpupdate /force` sonrası **Yönetim Şablonları → Windows Bileşenleri → winlrtRAR**.

## Hızlı test (OU / tek PC)

`scripts/winlrt-corporate-policy-sample.reg` — `reg import` veya çift tık (HKLM).

Doğrulama: `scripts/windows/corporate-pilot-preflight.ps1`

## Vitrin ZIP

Build: `scripts/package-corporate-gpo-zip.sh` → `apps/web/public/downloads/winlrtRAR-corporate-gpo.zip`

winlrtZIP: [README-winlrtZIP.md](./README-winlrtZIP.md) · `scripts/package-winlrtzip-corporate-gpo-zip.sh`
