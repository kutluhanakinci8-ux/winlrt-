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

**Deploy:** Her faz sonrası `scripts/post-phase-deploy.sh` (VPS ortam değişkenleri).

## Dalga planı (sıra)

| Dalga | Durum | Sonraki |
|-------|--------|---------|
| **1** CI VPS secret | Agent deploy OK · GitHub **bekliyor** | [DALGA1_GITHUB_VPS_SECRET.md](./DALGA1_GITHUB_VPS_SECRET.md) |
| **2** Explorer QA | Windows pilot | [qa/DALGA2_EXPLORER_QA_KOPYA.md](./qa/DALGA2_EXPLORER_QA_KOPYA.md) |
| **3** Kurumsal + imza | Faz 2 sonrası | [qa/FAZ5_KURUMSAL_BASLANGIC.md](./qa/FAZ5_KURUMSAL_BASLANGIC.md) |
| **4** v1.0 | Faz 3 sonrası | [qa/FAZ6_V1_KAPOR.md](./qa/FAZ6_V1_KAPOR.md) |
