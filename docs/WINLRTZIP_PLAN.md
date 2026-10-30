# winlrtZIP — rekabet odaklı ürün planı (faz faz)

**Ürün adı:** winlrtZIP  
**Hedef:** WinZip, Windows yerleşik ZIP, Bandizip “ZIP modu” ve kurumsal sıkıştırma araçlarıyla **doğrudan rekabet**; ZIP/ZIP64 paylaşımında **daha hızlı, daha güvenli, daha şeffaf** deneyim.  
**Kod:** winlrtRAR ile **aynı Rust motoru** (`crates/winlrt-archive`); kabuk, varsayılan profiller ve pazarlama **ZIP odaklı**.

**Başlangıç kuralı (onaylı):** Kod geliştirmesi **winlrtRAR Faz 2 kapısı** geçilene kadar yalnızca plan / motor paylaşımı; ayrı ürün kabuğu Faz 2 sonrası hızlandırılır.  
**Platform sırası:** Windows → macOS → mobil (Faz 6+) → Lerta posta eki (Faz 8).

Vitrin: [winlrt.com.tr](https://winlrt.com.tr) · Kardeş ürün: [WINLRTRAR_PLAN.md](./WINLRTRAR_PLAN.md) · Özet: [PRODUCT_ROADMAP.md](./PRODUCT_ROADMAP.md)

---

## 1. Rekabet stratejisi (tek cümle)

Kullanıcı **her yerde açılan ZIP/ZIP64** üretsin ve açsın; kurum içi ve yüksek güvenlikte **`.wlrt`**; WinZip’in abonelik / karmaşık lisans modeli olmadan **tek sefer kurulum + kurumsal toplu lisans** ile kalsın.

---

## 2. Rakiplere göre üstünlük hedefleri

winlrtRAR ile **aynı KPI ailesi** (hepsi **P0**, Faz 3’te ölçüm):

| Alan | WinZip / tipik zayıflık | winlrtZIP hedefi |
|------|-------------------------|------------------|
| **Hız** (P0) | Arayüz ağırlığı, bulut upsell | Çok çekirdek ZIP aç/yaz; büyük arşivde **≥ %30 daha hızlı** (benchmark) |
| **Sıkıştırma** (P0) | Eski deflate varsayılanları | ZIP64 + modern preset; aynı içerikte **≥ %10 daha küçük** veya eşit boyutta daha hızlı |
| **Güvenlik** (P0) | ZipCrypto varsayılanı, zayıf parola UX | **AES-256** (WinZip uyumlu profiller) + Argon2id; ZipCrypto yalnızca “uyumluluk” modu |
| **Güven / bütünlük** | Sınırlı kurumsal denetim | İmzalı arşiv + denetim günlüğü (Faz 4) |
| **UX** | Abonelik ekranları, paket karmaşası | Modern UI, TR/EN, **abonelik zorunluluğu yok** |
| **Kurumsal** | Lisans portalı karmaşası | MSI/MSIX, GPO, sessiz kurulum, banka/sendika/KOBİ pilot |
| **Paylaşım** | WinZip Share bulut kilidi | Önce **yerel + e-posta eki profili**; bulut opsiyonel (Faz 5, KVKK odaklı) |
| **Lerta** (Faz 8) | Yok | Posta eki boyut limiti + optimum ZIP/`.wlrt` — rakipte yok |

**Ölçüm:** winlrtRAR ile ortak `benchmarks/`; ZIP çıktıları ayrı rapor satırı.

---

## 3. Format ve rekabet

| Format | winlrtZIP rolü | Yaklaşım |
|--------|----------------|----------|
| **ZIP / ZIP64** | **Ana yazma / okuma** | Evrensel paylaşım, e-posta eki profili |
| **`.wlrt`** | Kurum içi “en iyi” format | winlrtRAR ile aynı motor; ZIP ürününde “Güvenli arşiv” preset |
| **GZIP / TAR** | Geliştirici / sunucu | Okuma + sınırlı yazma |
| **7z, RAR** | Geçiş | **Okuma** (motor winlrtRAR Faz 2–3’ten miras); winlrtZIP UI’da ikincil |
| **RAR yazma** | — | **Hayır** (tescilli) |

**Pazarlama:** “WinZip kadar tanıdık ZIP, daha hızlı ve kurumsal güvene hazır.”

---

## 4. Üstün özellik omurgası

### Çekirdek (ortak motor, ZIP preset)

- [ ] ZIP64, çok parçalı arşiv (split) — Faz 2
- [ ] Profiller: **E-posta eki**, **Hızlı**, **Dengeli**, **Maksimum**, **Kurumsal .wlrt**
- [ ] Akışlı açma / tek dosya çıkarma (büyük ZIP)
- [ ] Bozuk ZIP’te kısmi kurtarma (Faz 5)

### Güvenlik

- [x] AES-256 şifreli ZIP (WinZip uyumlu interop testleri) — [MADDE_W8](./qa/MADDE_W8_WINLRTZIP_AES_INTEROP.md)
- [ ] Kurumsal parola politikası (Faz 4)
- [ ] İmzalı arşiv (Faz 4)

### Arayüz (winlrtZIP kabuğu)

- [ ] WinZip / Explorer alışkanlıkları: sürükle-bırak, sağ tık kabuk entegrasyonu (Faz 2–3)
- [ ] “WinZip’ten geçiş” sihirbazı (Faz 3)
- [ ] İş kuyruğu, karanlık tema

### Kurumsal

- [ ] Banka + sendika + KOBİ pilot paketi (winlrtRAR ile aynı segment)
- [ ] Denetim günlüğü, sessiz kurulum

### Lerta (Faz 8)

- [ ] Posta eki: otomatik ZIP boyut hedefi (ör. 10 / 25 MB)
- [ ] Mesaj eki: `.wlrt` veya AES-ZIP profili

---

## 5. Teknoloji

| Katman | Seçim |
|--------|--------|
| UI | Tauri 2 + React/TypeScript — `apps/winlrt-zip/` |
| Motor | `crates/winlrt-archive` (winlrtRAR ile **aynı**) |
| CLI | `winlrt-zip` (clap) — Faz 1 |
| Dağıtım | İmzalı Windows installer; notarized macOS |

```text
apps/winlrt-zip/             # winlrtZIP kabuk (Faz 2+)
apps/winlrt-rar/             # winlrtRAR kabuk
crates/winlrt-archive/       # Ortak motor
benchmarks/                  # WinZip / 7-Zip ZIP karşılaştırma
docs/WINLRTZIP_PLAN.md       # Bu dosya
```

---

## 6. Faz planı (winlrtRAR Faz 2 sonrası hızlanır)

### Faz 0 — Plan ve KPI kilidi (**şimdi**)

**Çıktılar**

- [x] Rekabet hedefi (WinZip odaklı)
- [x] KPI: hız / sıkıştırma / kurumsal güven — **üçü P0**
- [x] Başlangıç kilidi: winlrtRAR **Faz 2 kapısı**
- [x] WinZip özellik matrisi — [WINZIP_FEATURE_MATRIX.md](./WINZIP_FEATURE_MATRIX.md)
- [x] ZIP interop test listesi — [qa/WINLRTZIP_INTEROP_SCENARIOS.md](./qa/WINLRTZIP_INTEROP_SCENARIOS.md)

**Kapı:** [WINLRTZIP_PHASE0.md](./WINLRTZIP_PHASE0.md) onaylı → geliştirme **RAR F2 sonrası** Faz 1.

---

### Faz 1 — Motor: ZIP mükemmelliği (RAR F2 kapısı sonrası)

**Ön koşul:** winlrtRAR Faz 2 tamam (RAR açma + Windows MVP motor entegrasyonu).

- [x] ZIP/ZIP64 yaz/oku (I3 smoke + seyrek >4GiB test) — [MADDE_W10](./qa/MADDE_W10_WINLRTZIP_ZIP64.md)
- [x] CLI `winlrt-zip`: create, extract, list, test, bench — [MADDE_W1](./qa/MADDE_W1_WINLRTZIP_CLI.md)
- [x] E-posta eki profili (boyut + hız hedefi) — [MADDE_W7](./qa/MADDE_W7_WINLRTZIP_EMAIL.md)
- [x] Benchmark: WinZip / 7-Zip ZIP satırı CI’da — [MADDE_W6](./qa/MADDE_W6_WINLRTZIP_BENCH.md)

**Kapı:** ZIP benchmark raporu + CLI stabil.

---

### Faz 2 — Windows winlrtZIP (Tauri)

- [x] `apps/winlrt-zip` MVP: liste, çıkar, sıkıştır — [MADDE_W2](./qa/MADDE_W2_WINLRTZIP_TAURI.md)
- [x] Parola UI + Explorer `--secure` — [MADDE_W5](./qa/MADDE_W5_WINLRTZIP_PASSWORD.md)
- [x] Explorer sağ tık (temel) — [MADDE_W4](./qa/MADDE_W4_WINLRTZIP_EXPLORER.md)
- [x] winlrt.com.tr indirme kanalı — W3 vitrin

**Kapı:** Windows pilot (KOBİ) en az 1 senaryo.

---

### Faz 3 — macOS + geçiş

- [ ] macOS build, notarization
- [ ] WinZip geçiş sihirbazı
- [ ] KPI kapısı: üç P0 metrik raporu

**Kapı:** macOS + Windows benchmark yeşil.

---

### Faz 4 — Kurumsal

- [ ] GPO/MDM, denetim günlüğü, imzalı arşiv
- [ ] Banka + sendika pilot genişlemesi

**Kapı:** Kurumsal checklist imzalı.

---

### Faz 5 — Gelişmiş ZIP

- [ ] Split ZIP, onarım, solid benzeri optimizasyonlar
- [ ] Opsiyonel güvenli paylaşım (self-hosted / KVKK; WinZip Share alternatifi)

---

### Faz 6 — Mobil (okuyucu / sınırlı oluşturma)

- [ ] Android / iOS: aç, paylaş, Lerta’ya hazırlık

---

### Faz 7 — Mağaza olgunluğu

- [ ] Play / App Store politikaları, güncelleme hattı

---

### Faz 8 — Lerta entegrasyonu

- [ ] Posta eki otomasyonu (winlrtRAR planı ile ortak)

---

## 7. winlrtRAR ile ilişki

| | winlrtRAR | winlrtZIP |
|---|-----------|-----------|
| Birincil rakip | WinRAR, 7-Zip | **WinZip** |
| Birincil yazma | `.wlrt` + ZIP | **ZIP / ZIP64** |
| RAR açma | **Evet** (Faz 2) | Miras (motor) |
| Geliştirme sırası | **Önce** (Faz 1–2 aktif) | **Faz 0 plan; kod F1 RAR F2 sonrası** |
| Motor | `winlrt-archive` | **Aynı** |

---

## 8. Riskler

| Risk | Azaltma |
|------|---------|
| İki ürün kapsamı | Ortak motor; ZIP kabuğu RAR F2’den sonra |
| WinZip marka alışkanlığı | Geçiş sihirbazı + Explorer entegrasyonu |
| ZipCrypto güvensizliği | Varsayılan AES; ZipCrypto uyarı etiketi |
| Bulut beklentisi | Faz 5’te KVKK odaklı opsiyonel |

---

## 9. Onaylı kararlar (winlrtRAR ile hizalı, 2026-10-30)

| # | Konu | Karar |
|---|------|--------|
| 1 | KPI | **Hız + sıkıştırma + kurumsal güven — üçü P0** |
| 2 | Pilot | **Banka + sendika + KOBİ** |
| 3 | Kod başlangıcı | **winlrtRAR Faz 2 bitene kadar yalnızca plan + ortak motor** |
| 4 | RAR | winlrtZIP **RAR üretmez**; açma motor mirası |

Detay: [WINLRTZIP_PHASE0.md](./WINLRTZIP_PHASE0.md)
