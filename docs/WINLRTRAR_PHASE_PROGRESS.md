# winlrtRAR — faz ilerlemesi (agent)

Otomatik takip: [WINLRTRAR_ROADMAP_TO_100.md](./WINLRTRAR_ROADMAP_TO_100.md)

| Faz | Maddeler | Agent durumu | Not |
|-----|----------|--------------|-----|
| 0 | A, B | Tamam | `main` + Main pipeline |
| 1 | E, F | F agent / E kısmi | Agent rsync (ortam VPS_*); GitHub `VPS_SSH_PASSWORD` hâlâ yok → CI skip |
| 2 | C, G | Tamam + yenileme | Flag push ile Windows bench tetiklenir |
| 3 | H, D, I | **Kısmi** | Beta CI **yeşil** (`1025c0d`, NSIS `.exe` artifact); H=%0 imza yok; Release dispatch ile |
| 4 | J | **Başladı** | Release push tetikleniyor; pilot: [qa/FAZ4_PILOT_BASLANGIC.md](./qa/FAZ4_PILOT_BASLANGIC.md) |
| 5 | K | **Siz** | ADMX/.reg saha + [WINLRTRAR_PILOT_KICKOFF.md](./WINLRTRAR_PILOT_KICKOFF.md) |
| 6 | L | **Siz** | §7 üç kapı + tag `v1.0.0` |

**Deploy:** Her faz sonrası `scripts/agent-deploy-vitrin.sh` (VPS ortam değişkenleri).
