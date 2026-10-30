# winlrtZIP — macOS imza ve notarization (W14+)

Imzasiz **dmg** beta CI’da uretilir. Gatekeeper uyumu icin GitHub Actions secret:

| Secret | Aciklama |
|--------|----------|
| `APPLE_CERTIFICATE` | Developer ID Application (.p12) base64 |
| `APPLE_CERTIFICATE_PASSWORD` | P12 parolasi |
| `APPLE_SIGNING_IDENTITY` | Orn. `Developer ID Application: ...` |
| `APPLE_ID` | Apple ID e-posta |
| `APPLE_PASSWORD` | App-specific password |
| `APPLE_TEAM_ID` | Team ID |

Workflow: [winlrt-zip-macos.yml](../../.github/workflows/winlrt-zip-macos.yml)

Release (istege): `.github/flags/trigger-zip-macos-release` push → `winlrt-zip-macos-beta.<run>` tag.
