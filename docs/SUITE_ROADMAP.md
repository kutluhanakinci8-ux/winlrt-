# winlrt suite — birleşik yol haritası

**Operasyon:** `main` dalına merge → GitHub Actions vitrin deploy ([AUTO_DEPLOY.md](./AUTO_DEPLOY.md)).  
**Kod:** Tek monorepo; ortak motor, ayrı ürün kabukları.

## Vizyon (kısa)

| Katman | Açıklama |
|--------|----------|
| **winlrt.com.tr** | Yerli arşiv, belge, sertifika ailesi — kurumsal + bireysel |
| **lerta.com.tr** | Mesaj / posta (ayrı ürün); olgun winlrt modülleri **Faz 8** entegrasyonu |

## Ürün sırası (onaylı)

```mermaid
flowchart LR
  subgraph now [Şimdi]
    R1[winlrtRAR F1 motor]
    Z0[winlrtZIP F0 plan]
  end
  subgraph gate [Kapı]
    R2[winlrtRAR F2 RAR aç + Win MVP]
  end
  subgraph zip [winlrtZIP]
    Z1[F1 ZIP motor CLI]
    Z2[F2 Windows kabuk]
    Z3[F3 macOS KPI]
  end
  subgraph later [Sonra]
    M[Mobil F6–7]
    D[Belge güven F3 suite]
    L[Lerta F8]
  end
  R1 --> R2
  Z0 --> R2
  R2 --> Z1 --> Z2 --> Z3
  Z3 --> M
  Z3 --> D
  R2 --> L
  Z3 --> L
```

## Detay dokümanlar

| Ürün | Plan | Faz 0 |
|------|------|--------|
| winlrtRAR | [WINLRTRAR_PLAN.md](./WINLRTRAR_PLAN.md) | [WINLRTRAR_PHASE0.md](./WINLRTRAR_PHASE0.md) (kapalı) |
| winlrtZIP | [WINLRTZIP_PLAN.md](./WINLRTZIP_PLAN.md) | [WINLRTZIP_PHASE0.md](./WINLRTZIP_PHASE0.md) (açık) |

## Ortak KPI (P0 × 3)

1. **Hız** — benchmark CI  
2. **Sıkıştırma oranı** — `.wlrt` ve ZIP satırları  
3. **Kurumsal güven** — AES-256-GCM, Argon2id, denetim, imzalı kurulum  

Pilot: **banka, sendika, KOBİ**.

## Repo yapısı (hedef)

```text
apps/web/                 # winlrt.com.tr vitrin
apps/winlrt-rar/          # Faz 2+
apps/winlrt-zip/          # ZIP F2+
crates/winlrt-archive/      # Ortak motor (aktif)
benchmarks/
docs/                     # Planlar
.github/workflows/        # CI + deploy
```

## Vitrin faz özeti (kullanıcıya dönük)

| Adım | İçerik |
|------|--------|
| 1 | **winlrtRAR** motor + masaüstü; **winlrtZIP** planlı, RAR F2 sonrası hızlanır |
| 2 | Mobil mağazalar |
| 3 | PDF, imza, sertifika |
| 4 | Lerta üretim entegrasyonu |

Genel özet: [PRODUCT_ROADMAP.md](./PRODUCT_ROADMAP.md)
