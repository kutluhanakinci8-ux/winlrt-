# Lertiva Zip — mobil okuyucu PoC (G6)

**Durum:** Repo kapısı W26 + W36; native kabuk henüz ayrı repo/sprint.

## Sözleşme (W26)

- `lertiva-zip mobile-capabilities`
- `lertiva-zip mobile-inspect <archive.zip>`
- Tauri komutları: `mobile_capabilities`, `mobile_inspect_zip`

## Hedef kabuk

| Platform | Bundle / id | Not |
|----------|-------------|-----|
| Android | `com.lertiva.zip` | Play internal |
| iOS | `com.lertiva.zip` | TestFlight |

Tauri 2 mobile veya ince Flutter/React Native kabuk — W26 JSON API tüketir.

## Doğrulama

`bash scripts/verify-madde-w36.sh`
