# winlrtRAR — faz ilerlemesi (agent)

Otomatik takip: [WINLRTRAR_ROADMAP_TO_100.md](./WINLRTRAR_ROADMAP_TO_100.md)

| Faz | Maddeler | Agent durumu | Not |
|-----|----------|--------------|-----|
| 0 | A, B | Tamam | `main` + Main pipeline |
| 1 | E, F | Kısmi (~F 100 agent) | Her faz sonrası agent rsync; GitHub Secret 403 → CI deploy sizin panel |
| 2 | C, G | Tamam (agent) | Artifact `41045481805` → `windows-*-kpi.json`; resmi re-run dispatch 403 |
| 3 | H, D, I | **Blok** | Agent: workflow dispatch 403; `TAURI_SIGNING_*` verilmedi → Mac `scripts/mac-set-github-tauri-signing-secrets.sh` + Actions **winlrtRAR Windows beta build** |
| 4 | J | **Siz** | [qa/EXPLORER_QA_SIGNOFF.md](./qa/EXPLORER_QA_SIGNOFF.md) doldur + commit |
| 5 | K | **Siz** | ADMX/.reg saha + [WINLRTRAR_PILOT_KICKOFF.md](./WINLRTRAR_PILOT_KICKOFF.md) |
| 6 | L | **Siz** | §7 üç kapı + tag `v1.0.0` |

**Deploy:** Her faz sonrası `scripts/agent-deploy-vitrin.sh` (VPS ortam değişkenleri).
