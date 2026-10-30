# Madde Q2 — Fast hız KPI performans (sertifika yok)

## Hedef

`speed_gate: true` — tüm fixture’larda .wlrt açma, referansın **≥ %30** hızlısı ([bench KPI](../WINLRTRAR_PHASE0.md)).

## Yapılan (agent)

| Değişiklik | Dosya |
|------------|--------|
| Açma: dizinleri toplu oluştur | `wlrt.rs` |
| Küçük arşivde sıralı açma (<256 KiB payload) | `wlrt.rs` |
| `BufWriter` 64 KiB | `wlrt.rs` |
| Fast profil zstd seviye **0** (daha hızlı açma) | `profile.rs` |
| Extract ölçümü 5× min | `bench.rs` |

## Kalan

- **Windows resmi run [41095710700](https://github.com/kutluhanakinci8-ux/winlrt-/actions/runs/41095710700):** `text` ✅ · **`mixed`** 12 ms vs zip 17 ms (kapı için ≤11 ms — 1 ms gap).
- Paralel açma eşiği **ham byte** toplamına göre düzeltildi (`wlrt.rs`) — sonraki bench ile doğrulanacak.

Doğrulama (kod): `bash scripts/verify-madde-q2.sh`  
Resmi kapı: `bash scripts/agent-madde-q2-bench.sh` → Windows bench + vitrin import.
