# Post-v1 — sıradaki maddeler (A→L bitti)

A→L **%100** (imzasız resmi v1.0.0). Sıra:

| Sıra | Madde | Hedef | Doğrulama |
|------|-------|-------|-----------|
| **M** | v1.0.1 Authenticode pisti | Prep **%100** | `verify-madde-m.sh` |
| **N** | Windows KPI vitrin yenileme | **%100** | `verify-madde-c.sh` + samples |
| **O** | Beta Release yenileme | **%100** | `verify-madde-i.sh` (son beta tag) |
| **P** | Doküman / faz senkron | **%100** | PHASE_PROGRESS + AGENT_IZI güncel |

Strict imzalı kurulum: panel `TAURI_SIGNING_*` → `verify-madde-m-strict.sh` (secret yokken bilinçli FAIL).
