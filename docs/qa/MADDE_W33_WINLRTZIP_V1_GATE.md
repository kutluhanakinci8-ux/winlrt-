# Madde W33 — Lertiva Zip v1.0 GA kapı (Post-G1) %100

**Amaç:** Vitrin ve release politikasında Zip için **Beta 0.1.0 → v1.0.0 (kapı)** geçişinin ölçülebilir tanımı.

## Kapı maddeleri

| ID | Koşul | Kaynak |
|----|--------|--------|
| P0×3 | `all_pass: true`, platform `windows / x86_64` | `lertiva-zip-p0-gate.json` |
| W32 | Mağaza/mobil pilot hazırlık | `lertiva-zip-store-pilot-gate.json` |
| İmza hattı | Dev self-signed `.cer` + `TAURI_SIGNING_*` (G2) | `.github/signing/`, GitHub secrets |

**W33 `all_pass`:** P0 + W32 ✅ (imza hattı ayrı izlenir; EV/OV olmadan SmartScreen “bilinmeyen yayıncı” devam eder).

## Komutlar

```bash
bash scripts/evaluate-lertiva-zip-v1-gate.sh
bash scripts/verify-madde-w33.sh
```

## Vitrin

- `apps/web/public/benchmark/samples/lertiva-zip-v1-gate.json`
- Ana sayfa Zip satırı: `v1-gate.all_pass` → **v1.0.0 (kapı)**

## Sonraki (G2 / release)

- `lertiva-zip-windows` imzalı artifact + `v1.0.0` tag (veya imzalı `v1.0.1` politikası)
