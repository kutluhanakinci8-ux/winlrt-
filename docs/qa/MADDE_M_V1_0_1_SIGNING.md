# Madde M — v1.0.1 imzalı sürüm pisti (prep %100)

## Kapsam

Authenticode **henüz yok**; agent GitHub secret yazamaz (403). **Prep %100** = secret eklendiğinde tek komutla imzalı build + verify.

## Hazır bileşenler

| Bileşen | Konum |
|---------|--------|
| CI env (beta) | `.github/workflows/lertiva-arc-windows.yml` → `TAURI_SIGNING_*` |
| CI env (stable v1.0.x) | `.github/workflows/lertiva-arc-windows-stable.yml` → `WINDOWS_CERTIFICATE*` + `sign-windows-nsis-bundle.ps1` |
| Mac yükleme | `scripts/mac-set-github-tauri-signing-secrets.sh` |
| Doğrulama | `scripts/verify-windows-signature.ps1` |
| Rehber | [WINLRTRAR_CODE_SIGNING.md](../WINLRTRAR_CODE_SIGNING.md) |

## Panel (siz veya Mac script)

```bash
bash scripts/mac-set-github-tauri-signing-secrets.sh /yol/kurumsal.pfx
```

Sonra: `bash scripts/agent-madde-m-strict.sh` (imzalı beta/release koşusu + signtool).

## Tag politikası (Lertiva Arc installer adı)

- **v1.0.0** legacy (`winlrtRAR_*`) — dokunulmaz.
- **v1.0.1** Track A: `Lertiva Arc_*_x64-setup.exe` imzasız yayında.
- Imzalı **Lertiva Arc** kurulum: yeni tag (**`v1.0.2+`**) + stable workflow; bkz. [LERTIVA_ARC_RELEASE_TRACKS.md](./LERTIVA_ARC_RELEASE_TRACKS.md).

Doğrulama (prep): `bash scripts/verify-madde-m.sh`
