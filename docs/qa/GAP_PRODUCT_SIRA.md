# Gap G1–G10 sonrası — ürün kapanış sırası

Repo kapıları: `bash scripts/verify-faz-all.sh` ✅  
Ürün kapıları: `bash scripts/verify-product-gap-all.sh`

| Sıra | Faz | Agent / repo | İnsan |
|------|-----|--------------|--------|
| 1 | **G2 prod** | `lertiva-ev-ov-signing-gate.json` | EV/OV PFX → `WINDOWS_CERTIFICATE` · **[G2_G3_SIRALI_RUNBOOK.md](./G2_G3_SIRALI_RUNBOOK.md)** |
| 2 | **G3 notarize** | `lertiva-macos-notarize-gate.json` | `APPLE_*` (G2 sonrası) · aynı runbook Bölüm B |
| 3 | **G4** | signoff şablonları ✅ | Dış kullanıcı QA oturumu |
| 4 | **G5** | kurumsal paket ✅ | IT saha signoff |
| 5 | **G6** | mobil MVP ✅ (W26) | Android AAB sprint; **iOS ertele** |
| 6 | **G7** | listing gate ✅ | **Play internal URL only** — [G7_DESKTOP_FIRST_PLAY_INTERNAL.md](./G7_DESKTOP_FIRST_PLAY_INTERNAL.md) |
| 7 | **G8** | W31 staging ✅ | `LERTA_API_BEARER` prod |
| 8 | **G9** | motor + **Arc UI** tek dosya | solid/dedup R&D |
| 9 | **G10** | Tur4 + suite kickoff ✅ | PDF/imza modül kodu |

Flag dosyaları (ürün tamamlandığında):

- G2: `apps/web/public/benchmark/samples/lertiva-ev-ov-configured.flag` (içerik: `ev_ov`)
- G3: `apps/web/public/benchmark/samples/lertiva-apple-notarize-configured.flag` (`notarize`)
- G7: `lertiva-store-listing-url.txt` (Play internal URL; masaüstü önce)
