# winlrtRAR — durum özeti (Cloud Agent)

**Son güncelleme:** Faz 3–6 yol · **`main`** (son deploy commit: `e9d0215` civarı).

## Ne oldu?

| Alan | Durum |
|------|--------|
| Motor | `.wlrt`, ZIP AES, RAR/7z, benchmark KPI, mixed fixture |
| Explorer | NSIS hooks, çoklu seçim, Win32 parola diyaloğu |
| Kurumsal | ADMX + örnek `.reg`, audit forward |
| CI | Main pipeline + Windows benchmark + **beta build yeşil** |
| Beta | Release [winlrt-rar-beta.41064653814](https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/winlrt-rar-beta.41064653814) |
| Vitrin | winlrt.com.tr — KPI samples + `/downloads/` beta link |
| Deploy | **Agent otomatik rsync** (yetkili `VPS_*` — her push sonrası). GitHub CI: agent token secret yazamaz (403) |

## Merge

Tüm feature zinciri **`main`** atasları — [WINLRTRAR_BRANCH_MERGE.md](./WINLRTRAR_BRANCH_MERGE.md).

## Sıradaki (insan)

| Faz | Adım |
|-----|------|
| 4 | Windows pilot → [qa/FAZ4_PILOT_BASLANGIC.md](./qa/FAZ4_PILOT_BASLANGIC.md) → [EXPLORER_QA_SIGNOFF.md](./qa/EXPLORER_QA_SIGNOFF.md) |
| 5 | [qa/FAZ5_KURUMSAL_BASLANGIC.md](./qa/FAZ5_KURUMSAL_BASLANGIC.md) |
| 6 | [qa/FAZ6_V1_KAPOR.md](./qa/FAZ6_V1_KAPOR.md) |

İlerleme: [WINLRTRAR_PHASE_PROGRESS.md](./WINLRTRAR_PHASE_PROGRESS.md)

Tam checklist: [WINLRTRAR_PRODUCTION_CHECKLIST.md](./WINLRTRAR_PRODUCTION_CHECKLIST.md)  
A→L **%100 yol haritası:** [WINLRTRAR_ROADMAP_TO_100.md](./WINLRTRAR_ROADMAP_TO_100.md)
