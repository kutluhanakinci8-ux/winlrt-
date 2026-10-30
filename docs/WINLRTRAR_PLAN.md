# winlrtRAR — ürün planı (faz faz)

**Ürün adı:** winlrtRAR  
**Konumlandırma:** WinRAR sınıfı arşiv deneyimi — **%100 yerli kod** hedefi, kurumsal + bireysel.  
**Platform sırası:** Windows → macOS → (ileride mobil yardımcı / Lerta entegrasyonu)

Vitrin: [winlrt.com.tr](https://winlrt.com.tr) · Genel yol haritası: [PRODUCT_ROADMAP.md](./PRODUCT_ROADMAP.md)

---

## Kapsam özeti

| Hedef | Açıklama |
|--------|-----------|
| Kullanıcı deneyimi | Sıkıştır, aç, parola, bölünmüş arşiv, sürükle-bırak, sağ tık kabuk entegrasyonu (Windows) |
| Yerli kod | Sıkıştırma motoru ve arayüz bizim; üçüncü taraf yalnızca açık lisanslı bileşenler (aşağıda) |
| Uyumluluk | Yaygın **açık** formatlar; kullanıcıların mevcut dosyalarıyla çalışma |
| Kurumsal | Sessiz kurulum, GPO/MDM dağıtımı, güncelleme kanalı, denetim günlüğü (ileri faz) |
| Lerta | Faz 6+ — posta eki / mesaj eki akışları (ayrı ürün olgunlaştıktan sonra) |

### Format stratejisi (önemli)

**`.rar` formatı tescilli / lisanslıdır** (WinRAR ekosistemi). Tamamen “sıfırdan RAR yazmak” veya tescilli formatı klonlamak hukuki ve teknik risk taşır.

**winlrtRAR stratejisi (önerilen):**

1. **Ana (yerli) format:** örn. **`.wlrt`** — kendi konteyner + sıkıştırma pipeline’ımız (açık spesifikasyon, ileride Lerta ile paylaşılabilir).
2. **Okuma / yazma (açık formatlar):** ZIP, TAR, GZIP, (Faz 2+) 7z okuma veya sınırlı yazma — lisans şartlarına uygun kütüphaneler.
3. **RAR dosyaları:**  
   - **Faz 1–3:** RAR desteği **yok** veya yalnızca “RAR dosyası — destek yol haritasında” mesajı.  
   - **Faz 4 (isteğe bağlı):** Yalnızca **açma (read-only)** — resmi UnRAR lisans koşullarına uygun entegrasyon **veya** hukuki inceleme sonrası karar.  
   - **Asla iddia etmeyin:** “RAR’ın tescilli formatının yerli klonu” — pazarlama: **“WinRAR benzeri arayüz ve iş akışı”**.

Bu plan, “WinZip/WinRAR **gibi** program” hedefinizle uyumludur; format tarafında **yerli + açık standartlar** önceliklidir.

---

## Teknoloji çerçevesi (karar taslağı)

| Katman | Öneri | Gerekçe |
|--------|--------|---------|
| UI | **Tauri 2** + React/TypeScript | Küçük kurulum, Windows/macOS tek kod tabanı |
| Çekirdek | **Rust** (`crates/winlrt-archive`) | Performans, bellek güvenliği, paylaşımlı motor (winlrtZIP ile) |
| Sıkıştırma | **zstd** / **lz4** (yerli pipeline) | Modern, hızlı, açık |
| ZIP uyumu | `zip` crate veya benzeri | De-facto paylaşım formatı |
| Paketleme | MSI/MSIX (Win), signed .dmg/.pkg (Mac) | Kurumsal dağıtım |

**Monorepo yeri (ileride):**

```text
apps/winlrt-rar/          # Tauri kabuk, winlrtRAR markası
crates/winlrt-archive/    # Ortak arşiv motoru (winlrtZIP ile paylaşılır)
```

---

## Faz 0 — Ürün tanımı ve kabul kriterleri (1–2 hafta eşdeğeri iş)

**Çıktılar**

- [ ] Kullanıcı hikâyeleri (bireysel + kurumsal) — en az 10 madde
- [ ] MVP özellik listesi (Faz 1’de “evet/hayır” net)
- [ ] Format tablosu (v1 desteklenen / desteklenmeyen)
- [ ] `.wlrt` v0 spesifikasyon taslağı (sihirli byte, sürüm, sıkıştırma algoritması kodu)
- [ ] Hukuk / lisans notu (RAR, 7z, kullanılan crate lisansları)
- [ ] Rakip UX referans listesi (WinRAR akışları: Sihirbaz, Test, Onar, Parola)

**Kabul**

- Ekip (siz + geliştirme) “Faz 1’e gir” diye onaylar.

---

## Faz 1 — Çekirdek motor + CLI (MVP backend)

**Amaç:** Grafik arayüz yok; motor kanıtı.

**İş**

- [ ] Rust crate: tek dosya / klasör → `.wlrt` arşiv
- [ ] `.wlrt` açma, bütünlük kontrolü (checksum)
- [ ] ZIP **açma** + **yazma** (temel)
- [ ] TAR + GZIP açma
- [ ] Parola (Faz 1 sonu veya Faz 2 — AES-256 ile konteyner)
- [ ] CLI: `winlrt-rar create|extract|list|test`

**Kabul**

- Windows/macOS’ta CLI ile örnek klasör round-trip
- 1 GB test arşivi performans log’u (süre, CPU, bellek üst sınırı)

---

## Faz 2 — Windows masaüstü (winlrtRAR v0.1)

**Amaç:** İlk indirilebilir Windows sürümü (beta).

**İş**

- [ ] Tauri proje iskeleti `apps/winlrt-rar`
- [ ] Ana pencere: arşiv içeriği listesi, araç çubuğu (Ekle, Çıkar, Test, Sıkıştır)
- [ ] Sürükle-bırak, çift tıkla aç
- [ ] İlerleme çubuğu, iptal
- [ ] Ayarlar: varsayılan sıkıştırma seviyesi, hedef format (.wlrt / .zip)
- [ ] Basit hata mesajları (TR)
- [ ] Otomatik güncelleme taslağı (winlrt.com.tr feed — Faz 2 sonu)

**Kabul**

- 3 gönüllü senaryo: indir → kur → zip aç → wlrt oluştur → test
- winlrt.com.tr **İndir** sayfasında imzalı/sürüm numaralı `.exe` (beta)

---

## Faz 3 — Windows olgunlaştırma (v1.0)

**Amaç:** Günlük kullanım + kurumsal pilot.

**İş**

- [ ] Windows Explorer sağ tık menüsü (Kayıt defteri / kabuk genişletmesi)
- [ ] Çok ciltli arşiv (.wlrt001 …) veya ZIP split
- [ ] Parola / şifreli arşiv (UI)
- [ ] Onarım / test komutları (kendi formatında tutarlılık)
- [ ] Sessiz kurulum: `/S` veya MSIX
- [ ] Çökme raporu (opsiyonel, gizlilik politikası ile)
- [ ] winlrtZIP ile ortak motor sürümü kilitleme

**Kabul**

- 1 kurumsal pilot (KOBİ veya dernek): 10+ kullanıcı, geri bildirim listesi
- v1.0 sürüm notları vitrinde

---

## Faz 4 — macOS (v1.0 mac)

**Amaç:** Feature parity (kabuk farkları hariç).

**İş**

- [ ] Tauri macOS build, kod imzalama, notarization
- [ ] Finder entegrasyonu (Services veya “Open With”)
- [ ] Apple Silicon + Intel evrensel binary
- [ ] Platforma özgü kısayollar ve HIG uyumu

**Kabul**

- macOS 12+ test matrisi; dmg vitrinde yayın

---

## Faz 5 — Gelişmiş formatlar ve performans

**Amaç:** “Diğer programlarla uyumlu” beklentisi.

**İş**

- [ ] 7z **açma** (lisans uyumlu kütüphane)
- [ ] Büyük dosya / >4GB ZIP (Zip64)
- [ ] Çok iş parçacıklı sıkıştırma
- [ ] İsteğe bağlı: RAR **sadece açma** (hukuki onay + UnRAR lisans yolu) — **veya** bilinçli olarak erteleme
- [ ] `.wlrt` v1 (geriye dönük uyumluluk kuralı)

**Kabul**

- Format matrisi dokümante; regresyon test paketi CI’da

---

## Faz 6 — Kurumsal ölçek

**Amaç:** Banka / sendika / kamu dağıtımı.

**İş**

- [ ] Merkezi güncelleme sunucusu veya GitHub Releases + imza doğrulama
- [ ] Grup ilkesi: varsayılan şifre kuralları, log konumu
- [ ] MSI transform / kurumsal markalama (logo, varsayılan dil)
- [ ] Denetim: kim, ne arşivledi (opsiyonel, KVKK metni)

**Kabul**

- En az 1 kurumsal referans + dağıtım runbook’u

---

## Faz 7 — Lerta entegrasyonu

**Amaç:** Mesajlaşma ve posta hattında üretim.

**Ön koşul:** lerta.com.tr mesaj/posta MVP’si hazır; winlrtRAR v1+ kararlı.

**İş**

- [ ] Posta eki: otomatik `.wlrt` / zip paketleme API
- [ ] Mesaj eki boyut limiti / sunucu tarafı tarama uyumu
- [ ] Ortak kimlik (SSO) — Lerta hesabı ile lisans (ileride)

**Kabul**

- Lerta staging ortamında uçtan uca eklı dosya senaryosu

---

## winlrtZIP ile ilişki

| Bileşen | Paylaşım |
|---------|----------|
| `crates/winlrt-archive` | **Ortak** |
| Tauri kabuk | Ayrı uygulama (marka: winlrtRAR vs winlrtZIP) |
| Varsayılan preset | RAR-benzeri: daha yüksek sıkıştırma; ZIP-benzeri: hız + uyumluluk |

**Geliştirme sırası önerisi:** Motor (Faz 0–1) **bir kez** → winlrtRAR Windows (Faz 2–3) paralelde winlrtZIP’i aynı motor + farklı varsayılanlarla türetmek mümkün.

---

## Riskler

| Risk | Azaltma |
|------|---------|
| RAR beklentisi | Vitrin/dokümanda “WinRAR benzeri UX”; format tablosu şeffaf |
| Performans | Erken benchmark; zstd seviye ayarı |
| Kurumsal güven | İmza, SBOM, açık sürüm notları |
| Kapsam genişlemesi | Faz kapıları — MVP dışı özellik Faz 5+ |

---

## Sıradaki somut adım (sizin onayınızla)

1. **Faz 0 checklist** maddelerini birlikte doldurmak (MVP evet/hayır listesi).  
2. Repoda `apps/winlrt-rar` Tauri iskeleti + `crates/winlrt-archive` boş crate (Faz 1 başlangıcı).

**Karar soruları (cevaplayın, planı kilitleyelim):**

1. İlk beta yalnızca **Windows** mu? (öneri: evet)  
2. `.wlrt` uzantısı uygun mu, yoksa `.winlrt` / başka?  
3. RAR **açma** Faz 5’e mi kalsın, yoksa hiç olmasın mı?  
4. winlrtZIP ile **aynı anda** mı başlayalım, önce **sadece winlrtRAR** mı?
