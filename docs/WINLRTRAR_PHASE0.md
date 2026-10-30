# winlrtRAR — Faz 0 (kapatıldı)

Onay tarihi: 2026-10-30 · Plan: [WINLRTRAR_PLAN.md](./WINLRTRAR_PLAN.md)

## Onaylı ürün kararları

| Karar | Sonuç |
|--------|--------|
| Faz 2’de RAR açma (rekabet / geçiş) | **Evet** |
| KPI önceliği | **Hepsi P0** — hız, sıkıştırma oranı, kurumsal güven (paralel izleme) |
| Kurumsal pilot hedefi | **Banka + sendika + KOBİ** (üç segment) |
| winlrtZIP başlangıcı | **winlrtRAR Faz 2 bitene kadar bekler** |

## KPI hedefleri (Faz 3 kapısı — üçü birden)

| KPI | Hedef | Ölçüm |
|-----|--------|--------|
| Hız | “Hızlı aç” profilinde referans (7-Zip / WinRAR) ile aynı test setinde **≥ %30 daha hızlı** açma veya eşdeğer işlem | `benchmarks/` CI raporu |
| Sıkıştırma | Aynı test setinde ZIP çıktısına göre `.wlrt` **≥ %10 daha küçük** veya eşit boyutta **daha hızlı** | Aynı |
| Kurumsal güven | AES-256-GCM + Argon2id; denetim günlüğü tasarımı onaylı; imzalı kurulum | Faz 2–4 checklist |

## Format (v1 hedef)

| Format | Faz 1 | Faz 2 | Yazma |
|--------|-------|-------|--------|
| `.wlrt` | create/extract/list/test | UI | Evet (ana) |
| ZIP / Zip64 | evet | evet | Evet |
| RAR / RAR5 | — | **açma** | Hayır |
| 7z | — | açma | Hayır (Faz 5 yazma opsiyonel) |
| TAR/GZIP | açma | açma | GZIP/TAR sınırlı |

## Faz 0 checklist

- [x] Rekabet stratejisi ve KPI tanımı
- [x] Üstün özellik omurgası (plan §4)
- [x] Stakeholder kararları (4 soru)
- [x] Pilot segmentleri: banka, sendika, KOBİ
- [ ] Rakip özellik matrisi (WinRAR, WinZip, 7-Zip, Bandizip) — Faz 1 paralel
- [x] `.wlrt` v0 binary spec — [WLRT_FORMAT_V0.md](./WLRT_FORMAT_V0.md)
- [ ] RAR okuma hukuk/lisans notu (UnRAR yolu) — Faz 2 öncesi zorunlu

**Faz 0 kapısı:** Kararlar onaylı → **Faz 1 başladı** (`crates/winlrt-archive`).
