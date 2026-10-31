# AP-1 — Lertiva Arc Windows x64 imzalı (v1.0.2)

**Hedef:** GitHub Release’te **Authenticode imzalı** `Lertiva.Arc_1.0.2_x64-setup.exe` · vitrin `authenticode_signed: true`.

## Bölüm 1 — Siz (EV/OV sertifika)

1. Kurumsal **EV veya OV** code signing PFX edinin.
2. GitHub repo secret:
   ```bash
   bash scripts/mac-set-github-tauri-signing-secrets.sh /yol/kurumsal.pfx
   ```
   veya panel: `WINDOWS_CERTIFICATE` (base64 PFX) + `WINDOWS_CERTIFICATE_PASSWORD`.
3. (Opsiyonel ürün kapısı) Imzalı release doğrulandıktan sonra:
   ```bash
   echo ev_ov > apps/web/public/benchmark/samples/lertiva-ev-ov-configured.flag
   bash scripts/verify-ev-ov-flag-honesty.sh
   ```

## Bölüm 2 — Agent / CI

Repo hazır (AP-0 ✅):

- `tauri.conf.json` **1.0.2**
- `lertiva-arc-windows-stable.yml` → imza + `verify-windows-signature.ps1`
- `bash scripts/agent-arc-ap1-publish.sh v1.0.2`

## Bölüm 3 — Doğrulama

```bash
bash scripts/evaluate-lertiva-arc-platform-gate.sh
jq '.phases["AP-1"]' apps/web/public/benchmark/samples/lertiva-arc-platform-gate.json
bash scripts/refresh-lertiva-downloads-manifest.sh
bash scripts/verify-arc-platform-ap0.sh
```

**AP-1 `pass: true`** koşulu: `ev_ov` flag + GitHub’da **`v1.0.2`** stable release (tag = tauri sürümü).

## Bölüm 4 — Vitrin

- `npm run build:web` / deploy
- `/downloads/` — imza satırı manifest’ten
- SmartScreen hub — gerçek kurulum PNG: `scripts/import-real-installer-screenshots.sh`

**Sonraki faz:** [ARC_PLATFORM_FAZ_PLAN.md](./ARC_PLATFORM_FAZ_PLAN.md) **AP-2** (ARM64).
