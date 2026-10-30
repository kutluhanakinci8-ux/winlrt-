# Arşiv (eski / yedek kod)

Canlı geliştirme **`main`** dalında tek kaynaktır. Karışıklığı önlemek için:

| Kural | Açıklama |
|--------|-----------|
| **Aktif kod** | `apps/web`, `apps/winlrt-rar`, `crates/`, `deploy/` |
| **Arşiv** | Kullanımdan kalkan dosyalar buraya taşınır; silinmez |
| **Git geçmişi** | Asıl yedek; `archive/snapshots/` isteğe bağlı nokta yedek |

## snapshots/

`scripts/repo-archive-snapshot.sh` ile oluşturulur (yerel veya CI öncesi). Git’e nadiren commit edilir — büyük dosya eklemeyin.

## Taşınmış modüller

| Yol | Not |
|-----|-----|
| `archive/deprecated-apps/desktop/` | Eski `apps/desktop` yönlendirme metni |
