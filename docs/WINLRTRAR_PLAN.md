# winlrtRAR — rekabet odaklı ürün planı (faz faz)

**Ürün adı:** winlrtRAR  
**Hedef:** WinRAR, WinZip, 7-Zip, Bandizip ve benzeri programlarla **doğrudan rekabet**; tanımlı alanlarda **ölçülebilir şekilde üstün** olmak.  
**Kod:** Sıkıştırma motoru, arayüz ve kurumsal katman **yerli geliştirme**; pazarın eski lisans / deneme ekranı modelinin dışında, şeffaf dağıtım.

**Platform sırası:** Windows → macOS → mobil (Faz 6+) → Lerta üretim entegrasyonu (Faz 8).

Vitrin: [winlrt.com.tr](https://winlrt.com.tr) · Genel plan: [PRODUCT_ROADMAP.md](./PRODUCT_ROADMAP.md)

---

## 1. Rekabet stratejisi (tek cümle)

Kullanıcı **mevcut arşivlerini açabilsin** (ZIP, RAR, 7z, …), yeni arşivlerini ise **daha hızlı, daha güvenli ve daha akıllı** `.wlrt` formatında oluştursun — kurumsal ve bireysel kullanıcı WinRAR’a geri dönmek zorunda kalmasın.

---

## 2. Rakiplere göre üstünlük hedefleri

Her faz sonunda aşağıdaki “kazanma kriterleri”nden en az birkaçı **benchmark veya pilot** ile kanıtlanır.

| Alan | WinRAR / WinZip tipik zayıflık | winlrtRAR hedefi |
|------|-------------------------------|------------------|
| **Hız** (P0) | Eski algoritmalar, tek çekirdek ağırlığı | Çok çekirdek + modern codec (zstd/lz4); “Hızlı aç” modunda rakiplerden **≥ %30 daha hızlı** (Faz 3 benchmark) |
| **Sıkıştırma** (P0) | RAR iyi ama kapalı; ZIP zayıf | `.wlrt` ile aynı dosyada rakip ZIP’e göre **≥ %10 daha küçük** veya eşit boyutta **daha hızlı** (Faz 3) |
| **Güvenlik** (P0) | Eski ZIP şifreleme, zayıf varsayılanlar | AES-256-GCM, Argon2id parola türetme; kurumsal “şifre politikası” (Faz 2–4) |
| **Güven / bütünlük** | Sınırlı imza deneyimi | Arşiv **dijital imza** + checksum ağacı; kurum içi “sahte arşiv” tespiti (Faz 4) |
| **UX** | Eski arayüz, deneme uyarıları | Modern UI, karanlık tema, sürükle-bırak, **deneme ekranı yok**, TR/EN (Faz 2+) |
| **Kurumsal** | Lisans karmaşası, sınırlı denetim | Sessiz kurulum, MDM/GPO, merkezi güncelleme, **KVKK uyumlu denetim günlüğü** (Faz 5–6) |
| **Otomasyon** | CLI var ama sınırlı entegrasyon | Tam CLI + **JSON API** + script örnekleri (banka/servis) (Faz 4) |
| **Ekosistem** | Bağımsız araç | **Lerta** posta/mesaj: tek tık optimum eklı arşiv (Faz 8) — rakiplerde yok |
| **Geçiş** | Kilitleme hissi | “WinRAR’dan geçiş” sihirbazı, sağ tık aynı kısayollar (Faz 3) |

**Ölçüm:** `benchmarks/` klasöründe sabit test seti (dosya türleri: metin, ofis, foto, video örnekleri); her release’te CI raporu.

---

## 3. Format ve rekabet (hukuk + pazar)

| Format | Rekabet rolü | Yaklaşım |
|--------|--------------|----------|
| **`.wlrt`** | **Ana silah** — yazma + okuma, en iyi performans/güvenlik | Tamamen yerli spesifikasyon; açık sürüm numarası |
| **ZIP / ZIP64** | Evrensel paylaşım | Okuma + yazma; e-posta eki profili |
| **7z** | Rakip kullanıcı kazanımı | Okuma (Faz 3); yazma opsiyonel (Faz 5) |
| **RAR / RAR5** | Geçiş zorunluluğu | **Okuma** (Faz 2–3): lisanslı/hukuki yoldan — kullanıcı mevcut arşivini açar, yeni iş **.wlrt** |
| **TAR, GZIP, BZ2** | Sunucu / geliştirici | Okuma; gerektiğinde yazma |

**Pazarlama dili:** “WinRAR’dan daha modern, daha güvenli, daha hızlı” — **yazma formatı `.wlrt`**, RAR **üretmeyiz** (tescilli); RAR **açarız** (rekabet için geçiş).

---

## 4. Üstün özellik listesi (ürün omurgası)

Bunlar “nice to have” değil; faz planına **yayılır**.

### Çekirdek (motor)

- [ ] Çok çekirdekli sıkıştırma / açma pipeline
- [ ] Codec seçimi: **Hız / Dengeli / Maksimum / E-posta eki** tek tık
- [ ] **Akışlı açma:** büyük arşivde tek dosyayı diske taşmadan önizleme / çıkarma
- [ ] **Solid + deduplication** (Faz 5): benzer dosyalarda boyut avantajı
- [ ] Bozuk arşivde **kısmi kurtarma** (Faz 5)

### Güvenlik

- [ ] Parola + anahtar dosyası (kurumsal)
- [ ] İmzalı arşiv (kurum sertifikası ile — Faz 4)
- [ ] Güvenli silme entegrasyonu noktası (opsiyonel hook)

### Arayüz

- [ ] Tek pencerede gezinme (tree + liste), arama, filtre
- [ ] İş kuyruğu: birden çok arşiv sırayla, arka planda
- [ ] WinRAR kısayol haritası (Ctrl+A, Alt+E, …) — geçiş kolaylığı

### Kurumsal

- [ ] Politika: “Yalnızca .wlrt ile dışarı çıkar”, minimum parola uzunluğu
- [ ] Denetim günlüğü (kim, ne zaman, hangi arşiv — KVKK metni)
- [ ] MSI/MSIX, offline güncelleme paketi

### Lerta (Faz 8 — fark yaratıcı)

- [ ] Posta eki: otomatik boyut limiti + optimum sıkıştırma
- [ ] Mesaj eki: uçtan uca şifreli `.wlrt` profili

---

## 5. Teknoloji

| Katman | Seçim |
|--------|--------|
| UI | Tauri 2 + React/TypeScript |
| Motor | Rust `crates/winlrt-archive` (winlrtZIP ile ortak) |
| Codecs | zstd (kalite), lz4 (hız), zlib (ZIP uyumu) |
| Şifre | AES-256-GCM, Argon2id |
| Dağıtım | İmzalı Windows installer; notarized macOS |

```text
apps/winlrt-rar/           # winlrtRAR kabuk
crates/winlrt-archive/     # Rekabet motoru
benchmarks/                # Rakip karşılaştırma seti
```

---

## 6. Faz planı (rekabet sırasına göre)

### Faz 0 — Rekabet analizi ve KPI kilidi

**Çıktılar**

- [ ] Rakip matrisi (WinRAR, WinZip, 7-Zip, Bandizip): özellik tablosu
- [ ] **KPI hedefleri** (hız, oran, kurulum boyutu) — yukarıdaki tablo onaylı
- [ ] `.wlrt` v0 spesifikasyonu
- [ ] MVP: “Rakibe yetiş” vs “Rakibi geç” özellik listesi
- [ ] Hukuk: RAR okuma yolu, lisans listesi

**Kapı:** KPI’lar yazılı onay → Faz 1.

---

### Faz 1 — Motor: hız ve temel üstünlük

**Amaç:** Rakiplerle aynı işi **ölçülebilir** yapmak.

- [x] `.wlrt` create / extract / list / test / benchmark
- [x] ZIP okuma-yazma, Zip64 hazırlığı (eşik + smoke test)
- [x] Çok çekirdek sıkıştırma (`.wlrt` create, v1)
- [x] CLI + JSON (`--json`, `bench --all`)
- [x] CI **benchmark raporu** (artifact `bench-report`)
- [ ] Yerel rakip CLI karşılaştırması (7-Zip / WinRAR — aynı makine)

**Kapı:** Test setinde en az bir metrikte rakip referansına **eşit veya üstün**.

---

### Faz 2 — Windows beta: geçiş + üstün UX

**Amaç:** WinRAR kullanıcısını **deneyimle kazanmak**.

- [x] Tauri iskelet (`apps/winlrt-rar`), TR arayüz başlangıcı
- [x] **RAR5 okuma** (beta) — `rar` crate + CLI/Tauri; RAR4 / UnRAR statik link sırada
- [x] 7z okuma (`sevenz-rust`; parolalı → 7z CLI)
- [x] Sürükle-bırak + iş kuyruğu (Tauri beta)
- [x] Çift tık / dosya ilişkilendirme (beta: bundle `fileAssociations` + argv)
- [ ] Geçiş sihirbazı — varsayılan uygulama (Faz 3)
- [x] Profiller: Hızlı / Dengeli / Maksimum / E-posta (oluştur akışı)
- [x] winlrt.com.tr **beta indirme linki** (GitHub Releases)

**Kapı:** 10 gerçek kullanıcı “günlük işimi yapıyorum” anketi ≥ 8/10.

---

### Faz 3 — Windows 1.0: rekabet eşiği → üstünlük

**Amaç:** WinRAR’ı **varsayılan arşiv aracı** olmaktan çıkarmak.

Taslak: [WINLRTRAR_FAZ3_TRANSITION.md](./WINLRTRAR_FAZ3_TRANSITION.md)

- [ ] Explorer sağ tık (WinRAR benzeri menü)
- [ ] **Geçiş sihirbazı** (dosya ilişkilendirme, kısayollar) — uygulama içi özet (beta) mevcut
- [ ] Parola + güçlü şifreleme UI
- [ ] Çok ciltli arşiv, onarım/test
- [ ] Resmi benchmark yayını (vitrinde özet)
- [ ] v1.0 imzalı kurulum

**Kapı:** Benchmark KPI (Bölüm 2) **Windows’ta** karşılandı; 1 kurumsal pilot.

---

### Faz 4 — macOS 1.0 + otomasyon üstünlüğü

- [ ] macOS feature parity, notarization
- [ ] **JSON RPC / REST yerel API** (script, banka otomasyonu)
- [ ] Finder entegrasyonu
- [ ] İmzalı arşiv (kurumsal sertifika) v1

**Kapı:** macOS benchmark KPI; API dokümantasyonu yayında.

---

### Faz 5 — Performans ve format derinliği

- [ ] Solid arşivler, gelişmiş dedup
- [ ] Bozuk arşiv kurtarma modu
- [ ] 7z yazma (opsiyonel)
- [ ] `.wlrt` v1, geriye uyumluluk garantisi
- [ ] GPU hızlandırma **araştırma** (uygunsa Faz 5b)

**Kapı:** Büyük arşiv (>50 GB) senaryolarında stabilite testleri.

---

### Faz 6 — Kurumsal rekabet (banka, sendika, kamu)

- [ ] MDM/GPO, offline patch
- [ ] Denetim günlüğü, politika motoru
- [ ] Toplu lisans / kurum anahtarı
- [ ] SBOM, imza doğrulama, güvenlik whitepaper (TR)

**Kapı:** En az 1 banka veya sendika pilotu **imzalı** referans mektubu / iç onay.

---

### Faz 7 — Mobil mağazalar

- [ ] Android / iOS: açma, sınırlı oluşturma, paylaşım uzantıları
- [ ] Mağaza uyumluluk (Apple/Google kuralları)

**Kapı:** Store’da yayın; masaüstü ile format uyumu.

---

### Faz 8 — Lerta: rakiplerde olmayan katman

**Ön koşul:** Lerta mesaj/posta MVP + winlrtRAR v1+.

- [ ] Posta eki tek tık `.wlrt` optimum
- [ ] Mesaj eki güvenli profil
- [ ] Ortak hesap / lisans (isteğe bağlı)

**Kapı:** Lerta staging’de üretim senaryosu.

---

## 7. winlrtZIP ile ilişki

| | winlrtRAR | winlrtZIP |
|---|-----------|-----------|
| Konum | **Maksimum sıkıştırma + geçiş (RAR okuma)** | **Hız + evrensel ZIP paylaşımı** |
| Motor | Ortak `winlrt-archive` | Aynı |
| Rekabet | WinRAR, 7-Zip | WinZip |

**Onay:** Önce **winlrtRAR motor + Windows**; winlrtZIP aynı motor, ZIP odaklı preset — **Faz 2 kapısı sonrası** ([WINLRTZIP_PLAN.md](./WINLRTZIP_PLAN.md)).

---

## 8. Riskler

| Risk | Azaltma |
|------|---------|
| “Her alanda anında birinci” beklentisi | KPI’ları fazlara böl; benchmark şeffaf |
| RAR lisansı | Yalnızca okuma; yazma `.wlrt` |
| Kapsam patlaması | Her faz **kapı** kuralı; üstün özellik listesi öncelik puanı (P0/P1) |
| Güven algısı | Yerli kod, imza, kurumsal pilot |

---

## 9. Onaylı kararlar (2026-10-30)

| # | Soru | Karar |
|---|------|--------|
| 1 | Faz 2’de RAR açma | **Evet** |
| 2 | KPI önceliği | **Üçü birden P0:** hız, sıkıştırma, kurumsal güven |
| 3 | Kurumsal pilot | **Banka + sendika + KOBİ** |
| 4 | winlrtZIP | **Faz 2 bitene kadar bekler** |

Detay: [WINLRTRAR_PHASE0.md](./WINLRTRAR_PHASE0.md) · **Faz 1** kod: `crates/winlrt-archive`, `benchmarks/`.
