# Post-v1 — sıradaki maddeler (A→L bitti)

A→L **%100** (imzasız resmi v1.0.0). Sıra:

| Sıra | Madde | Hedef | Doğrulama |
|------|-------|-------|-----------|
| **M** | v1.0.1 Authenticode pisti | Prep **%100** ✅ | `verify-madde-m.sh` |
| **N** | Windows KPI vitrin yenileme | **%100** ✅ | bench run [41091547127](https://github.com/kutluhanakinci8-ux/winlrt-/actions/runs/41091547127) |
| **O** | Beta Release yenileme | **%100** ✅ | [winlrt-rar-beta.44](https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/winlrt-rar-beta.44) · run 41093461800 |
| **P** | Doküman / faz senkron | **%100** ✅ | PHASE_PROGRESS + AGENT_IZI |

Strict imzalı kurulum: panel `TAURI_SIGNING_*` → `verify-madde-m-strict.sh` (secret yokken bilinçli FAIL).
