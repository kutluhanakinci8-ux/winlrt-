# Madde J — Explorer pilot QA %100

## Tanım

1. [EXPLORER_QA_SIGNOFF.md](./EXPLORER_QA_SIGNOFF.md) → **[x] PASS**
2. Release SHA256 = GitHub asset digest (agent indirme doğrulaması)
3. `run-dalga-smoke.sh` + `cargo test -p lertiva-archive`

Saha UI tekrarı: `scripts/windows/pilot-dalga4-start.ps1` (Windows); agent kapısı repo+CI smoke ile kapanır.

Doğrulama: `bash scripts/verify-madde-j.sh`
