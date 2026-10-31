# Gap → %100 — ilerleme (agent)

| Faz | Repo kapısı | Ürün / insan | Doğrulama |
|-----|-------------|--------------|-----------|
| **G1** | P0 ✅ W33 ✅ W34 ✅ | — | `verify-faz-g1.sh` |
| **G2** | Imza CI ✅ vitrin PNG ✅ (incoming) | EV/OV prod imza | `verify-faz-g2.sh` |
| **G3** | W14 ✅ W34 ✅ | APPLE_* notarize | `verify-faz-g3.sh` |
| **G4** | Signoff şablonları ✅ | Dış kullanıcı oturumu | `verify-faz-g4.sh` |
| **G5** | Kurumsal repo ✅ | IT saha | `verify-faz-g5.sh` |
| **G6** | W36 ✅ mobil MVP JSON | apk/ipa internal | `verify-faz-g6.sh` |
| **G7** | W27+W32+Data safety ✅ | Play internal URL (masaüstü önce) | `verify-faz-g7.sh` · [G7_DESKTOP_FIRST](./G7_DESKTOP_FIRST_PLAY_INTERNAL.md) |
| **G8** | W31 staging ✅ | LERTA prod bearer | `verify-faz-g8.sh` |
| **G9** | Codec ✅ extract_one ✅ Arc UI tek dosya ✅ | solid/dedup R&D | `verify-faz-g9.sh` |
| **G10** | Tur4 policy ✅ brand scan ✅ suite kickoff ✅ | PDF/imza modülü | `verify-faz-g10.sh` |

Ürün sırası: [GAP_PRODUCT_SIRA.md](./GAP_PRODUCT_SIRA.md) · `verify-product-gap-all.sh`

**Lertiva Arc çoklu platform (henüz yapılmayan):** [ARC_PLATFORM_FAZ_PLAN.md](./ARC_PLATFORM_FAZ_PLAN.md) (AP-1 … AP-6).

**Durum raporu (%):** [GAP_STATUS_RAPOR.md](./GAP_STATUS_RAPOR.md) · `bash scripts/evaluate-gap-status-report.sh`

Son güncelleme: G1 platform polish (KPI runbook, macOS manifest) · sırada **G10** suite / kalan insan adımları.
