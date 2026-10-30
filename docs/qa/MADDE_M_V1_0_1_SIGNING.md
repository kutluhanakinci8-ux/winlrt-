# Madde M — v1.0.1 imzalı sürüm pisti (prep %100)

## Kapsam

Authenticode **henüz yok**; agent GitHub secret yazamaz (403). **Prep %100** = secret eklendiğinde tek komutla imzalı build + verify.

## Hazır bileşenler

| Bileşen | Konum |
|---------|--------|
| CI env | `.github/workflows/winlrt-rar-windows.yml` → `TAURI_SIGNING_*` |
| Mac yükleme | `scripts/mac-set-github-tauri-signing-secrets.sh` |
| Doğrulama | `scripts/verify-windows-signature.ps1` |
| Rehber | [WINLRTRAR_CODE_SIGNING.md](../WINLRTRAR_CODE_SIGNING.md) |

## Panel (siz veya Mac script)

```bash
bash scripts/mac-set-github-tauri-signing-secrets.sh /yol/kurumsal.pfx
```

Sonra: `bash scripts/agent-madde-m-strict.sh` (imzalı beta/release koşusu + signtool).

## v1.0.1 tag politikası

- **v1.0.0** dokunulmaz (imzasız resmi).
- Imzalı kurulum: yeni tag **`v1.0.1`** + Release NSIS (aynı sürüm numarası Tauri config ile uyumlu).

Doğrulama (prep): `bash scripts/verify-madde-m.sh`
