# Madde K — Kurumsal pilot %100

## Tanım

1. [KURUMSAL_PILOT_SIGNOFF.md](./KURUMSAL_PILOT_SIGNOFF.md) → **[x] PASS**
2. `bash scripts/verify-dalga3-prereqs.sh` (ADMX, GPO zip, pilot scriptleri)
3. `cargo test audit::` (denetim birimi)

Gerçek AD ortamı: `scripts/windows/pilot-dalga3-start.ps1` — opsiyonel tekrar.

Doğrulama: `bash scripts/verify-madde-k.sh`
