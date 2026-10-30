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

- **Linux `mixed`:** zip 4 ms / wlrt 3 ms → %30 kapısı için wlrt ≤2 ms gerekir (sınırda).
- **Windows `text`:** resmi koşuda tek kırmızı satır — yeni bench koşusu ile doğrulanacak.

Doğrulama (kod): `bash scripts/verify-madde-q2.sh`  
Resmi kapı: `bash scripts/agent-madde-q2-bench.sh` → Windows bench + vitrin import.
