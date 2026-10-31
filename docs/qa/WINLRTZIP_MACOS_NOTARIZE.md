# Lertiva Zip — macOS imza ve notarization (W14+)

Imzasiz **dmg** beta CI’da uretilir. Gatekeeper uyumu icin GitHub Actions secret:

| Secret | Aciklama |
|--------|----------|
| `APPLE_CERTIFICATE` | Developer ID Application (.p12) base64 |
| `APPLE_CERTIFICATE_PASSWORD` | P12 parolasi |
| `APPLE_SIGNING_IDENTITY` | Orn. `Developer ID Application: ...` |
| `APPLE_ID` | Apple ID e-posta |
| `APPLE_PASSWORD` | App-specific password |
| `APPLE_TEAM_ID` | Team ID |

Workflow: [lertiva-zip-macos.yml](../../.github/workflows/lertiva-zip-macos.yml)

Release (istege): `.github/flags/trigger-zip-macos-release` push → `winlrt-zip-macos-beta.<run>` tag.

## Adım adım (G3 ürün)

1. Developer ID Application .p12 + Team ID + app-specific password
2. GitHub’a 6 secret (tablo yukarı) — [G2_G3_SIRALI_RUNBOOK.md](./G2_G3_SIRALI_RUNBOOK.md) B2
3. Actions → **Lertiva Zip macOS beta build** → Run workflow
4. dmg: `spctl -a -vv -t install …` → notarized
5. `echo notarize > apps/web/public/benchmark/samples/lertiva-apple-notarize-configured.flag`
6. `bash scripts/evaluate-lertiva-macos-notarize-gate.sh`
