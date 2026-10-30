# winlrt — feature dal birleştirme sırası (Cloud Agent)

PR açılamıyorsa (`must be a collaborator`) **Compare & pull request** ile `main`’e sırayla merge edin (alttaki en eski → en yeni).

| Sıra | Dal | Odak |
|------|-----|------|
| 1 | `cursor/github-actions-and-env-ea18` | CI / env (taban) |
| 2 | `cursor/bench-winrar-kpi-ea18` | Benchmark KPI |
| 3 | `cursor/explorer-multiselect-ea18` | Explorer çoklu seçim |
| 4 | `cursor/secure-zip-argon-ea18` | ZIP AES, Argon2 |
| 5 | `cursor/explorer-password-policy-ea18` | Parola sihirbazı |
| 6 | `cursor/audit-native-password-ea18` | Win32 parola + audit |
| 7 | `cursor/gpo-naming-ea18` | GPO + adlandırma |
| 8 | `cursor/admx-log-forward-ea18` | ADMX + HTTP forward |
| 9 | `cursor/kpi-vitrin-signing-ea18` | Vitrin KPI + prod checklist |
| 10 | `cursor/fast-kpi-vitrin-import-ea18` | Hız KPI + vitrin import |
| 11 | `cursor/bench-large-kpi-explorer-ea18` | mixed fixture + Explorer QA |
| 12 | `cursor/signing-banner-status-ea18` | durum doc, imza Mac script, vitrin KPI şeridi |
| 13 | `cursor/post-merge-vitrin-ci-ea18` | post-merge rehber, vitrin KPI workflow, smoke |

**Tek compare (zincirin tamamı → main):**  
https://github.com/kutluhanakinci8-ux/winlrt-/compare/main...cursor/post-merge-vitrin-ci-ea18

Merge sonrası:

1. **Main pipeline** (push `main`)
2. **winlrt Windows benchmark (resmi)** — isteğe `KPI strict` veya Variable `WINLRT_KPI_STRICT=1`
3. **winlrtRAR Windows beta build**
4. `bash scripts/import-windows-kpi-to-vitrin.sh` → commit → deploy
5. [WINLRTRAR_EXPLORER_QA.md](./WINLRTRAR_EXPLORER_QA.md) pilot

İlgili: [WINLRTRAR_PRODUCTION_CHECKLIST.md](./WINLRTRAR_PRODUCTION_CHECKLIST.md)
