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

**Deploy:** Her faz sonrası `scripts/agent-deploy-vitrin.sh` (VPS ortam değişkenleri).
