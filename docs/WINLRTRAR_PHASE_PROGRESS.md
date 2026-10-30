# winlrtRAR — faz ilerlemesi (agent)

Otomatik takip: [WINLRTRAR_ROADMAP_TO_100.md](./WINLRTRAR_ROADMAP_TO_100.md)

| Faz | Maddeler | Agent durumu | Not |
|-----|----------|--------------|-----|
| 0 | A, B | Tamam | `main` + Main pipeline |
| 1 | E, F | F agent / E kısmi | Agent rsync (ortam VPS_*); GitHub `VPS_SSH_PASSWORD` hâlâ yok → CI skip |
| 2 | C, G | Tamam + yenileme | Flag push ile Windows bench tetiklenir |
| 3 | H, D, I | **Kısmi (~I ok)** | Release: `winlrt-rar-beta.41064653814`; artifact path düzeltildi; H imza yok |
| 4 | J | **Pilot PC** | Linux smoke OK ([qa/FAZ4_LINUX_SMOKE.md](./qa/FAZ4_LINUX_SMOKE.md)); Windows: [qa/FAZ4_PILOT_BASLANGIC.md](./qa/FAZ4_PILOT_BASLANGIC.md) |
| 5 | K | **Hazır** | Rehber: [qa/FAZ5_KURUMSAL_BASLANGIC.md](./qa/FAZ5_KURUMSAL_BASLANGIC.md) (Faz 4 signoff sonrası) |
| 6 | L | **Siz** | §7 üç kapı + tag `v1.0.0` |

**Her agent turu:** commit → `push origin main` → `scripts/post-phase-deploy.sh` ([AGENT_COMMIT_DEPLOY.md](./AGENT_COMMIT_DEPLOY.md)).

## Dalga planı (sıra)

| Dalga | Durum | Sonraki |
|-------|--------|---------|
| **1** Deploy | **Agent VPS deploy OK** (yetkili kimlik) · CI `auth=skip` = token 403, opsiyonel panel | [DALGA1_GITHUB_VPS_SECRET.md](./DALGA1_GITHUB_VPS_SECRET.md) |
| **2** Explorer QA | **Aktif** — Windows pilot + `explorer-qa-preflight.ps1` | CI: `verify-dalga2-prereqs.sh` |
| **3** Kurumsal + imza | **Sırada** (Dalga 2 signoff sonrası) | `verify-dalga3-prereqs.sh` · [qa/DALGA3_KURUMSAL_KOPYA.md](./qa/DALGA3_KURUMSAL_KOPYA.md) |
| **4** v1.0 | Faz 3 sonrası | [qa/FAZ6_V1_KAPOR.md](./qa/FAZ6_V1_KAPOR.md) |
