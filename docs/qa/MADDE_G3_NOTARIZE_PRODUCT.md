# G3 ürün — macOS notarize

Checklist: [WINLRTZIP_MACOS_NOTARIZE.md](./WINLRTZIP_MACOS_NOTARIZE.md)

1. `APPLE_*` secret set (6 adet)
2. Workflow: `lertiva-zip-macos.yml` — imzalı + notarize edilmiş dmg
3. Onay flag: `apps/web/public/benchmark/samples/lertiva-apple-notarize-configured.flag` içeriği `notarize`
4. `bash scripts/evaluate-lertiva-macos-notarize-gate.sh`

KPI bench notarize gerektirmez; imza ayrı kapı.
