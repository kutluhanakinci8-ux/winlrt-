# winlrtZIP — Faz 0 (planlama)

Onay tarihi: 2026-10-30 · Plan: [WINLRTZIP_PLAN.md](./WINLRTZIP_PLAN.md)

## Durum

**Faz 0 açık** — ürün kararları kilitlendi; **uygulama kodu** (ayrı kabuk) winlrtRAR **Faz 2 kapısı** sonrası başlar.  
Ortak motor (`crates/winlrt-archive`) winlrtRAR Faz 1 kapsamında gelişmeye devam eder; ZIP yetenekleri bu motorun parçasıdır.

## Onaylı ürün kararları

| Karar | Sonuç |
|--------|--------|
| Birincil rakip | **WinZip** (ZIP/ZIP64, kurumsal dağıtım) |
| KPI önceliği | **Hepsi P0** — hız, sıkıştırma, kurumsal güven |
| Kurumsal pilot | **Banka + sendika + KOBİ** |
| winlrtRAR ile sıra | **RAR Faz 2 kapısı → winlrtZIP Faz 1** |
| Birincil yazma formatı | **ZIP / ZIP64**; `.wlrt` kurumsal preset |
| RAR yazma | **Hayır** |

## KPI hedefleri (Faz 3 kapısı — winlrtRAR ile aynı ölçüm hattı)

| KPI | Hedef | Ölçüm |
|-----|--------|--------|
| Hız | Referans WinZip / 7-Zip ZIP ile aynı test setinde **≥ %30 daha hızlı** açma veya eşdeğer iş | `benchmarks/` ZIP satırı |
| Sıkıştırma | Aynı sette rakip ZIP’e **≥ %10 daha küçük** veya eşit boyutta daha hızlı | Aynı |
| Kurumsal güven | AES-256 ZIP + Argon2id; denetim tasarımı; imzalı kurulum | Faz 2–4 checklist |

## Format önceliği (winlrtZIP ürünü)

| Format | Faz 1 (motor) | Faz 2+ (UI) |
|--------|---------------|-------------|
| ZIP / ZIP64 | **P0** | **P0** |
| `.wlrt` | create/extract (motor) | “Güvenli arşiv” preset |
| GZIP / TAR | okuma | okuma |
| RAR / 7z | miras (RAR F2+) | ikincil menü |

## Faz 0 checklist

- [x] Rekabet stratejisi (WinZip)
- [x] KPI tanımı (3× P0)
- [x] Geliştirme kilidi (RAR F2)
- [x] winlrtRAR ile motor paylaşımı netleştirildi
- [ ] WinZip sürüm / özellik matrisi — Faz 1 paralel
- [ ] ZIP interop test senaryoları listesi — Faz 1 ilk sprint
- [ ] winlrt.com.tr vitrin metinleri (plan ile uyum) — sürekli

**Faz 0 kapısı (plan):** Bu doküman + [WINLRTZIP_PLAN.md](./WINLRTZIP_PLAN.md) onaylı → **kod Faz 1**, winlrtRAR **Faz 2 bittiğinde** resmi başlangıç.

## Suite zaman çizelgesi (özet)

```text
[Şimdi]     winlrtRAR Faz 1 — ortak motor + CLI
            winlrtZIP Faz 0 — plan (bu dosya)

[RAR F2]    RAR açma + winlrtRAR Windows MVP kapısı

[ZIP F1]    winlrtZIP motor/CLI sprint + ZIP benchmark

[ZIP F2–3]  winlrtZIP Windows/macOS kabuk + KPI kapısı

[Paralel]   Mobil, belge, Lerta — PRODUCT_ROADMAP.md
```

Tam suite: [SUITE_ROADMAP.md](./SUITE_ROADMAP.md)
