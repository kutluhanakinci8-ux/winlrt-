# G3 ürün — macOS notarize

**Sıra:** G2 EV/OV tamamlandıktan sonra — [G2_G3_SIRALI_RUNBOOK.md](./G2_G3_SIRALI_RUNBOOK.md) (Bölüm B)

Checklist detay: [WINLRTZIP_MACOS_NOTARIZE.md](./WINLRTZIP_MACOS_NOTARIZE.md)

1. `APPLE_*` secret set (6 adet)
2. Workflow: `lertiva-zip-macos.yml` — imzalı + notarize edilmiş dmg
3. Onay flag: `lertiva-apple-notarize-configured.flag` içeriği `notarize` (şablon: `.flag.example`)
4. `bash scripts/evaluate-lertiva-macos-notarize-gate.sh`

KPI bench notarize gerektirmez; imza ayrı kapı.
