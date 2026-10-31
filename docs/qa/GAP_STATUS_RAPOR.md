# Lertiva / winlrt — gap durum raporu (%)

**Tarih:** 2026-10-31 · **Dal:** `main` · **Ölçüm:** repo scriptleri + vitrin gate JSON + [GAP_TO_100_FAZ_PLAN.md](./GAP_TO_100_FAZ_PLAN.md)

**Ertelenen (bu raporda ayrı blok — sonra):** EV/OV PFX, Apple notarize, Google Play / App Store kayıt, Lerta prod bearer.

Otomatik özet JSON: `bash scripts/evaluate-gap-status-report.sh` → `lertiva-gap-status-report.json`

---

## 1) Genel özet

| Katman | Açıklama | Tamamlanma |
|--------|-----------|------------|
| **A — Repo & CI kapıları** | `verify-faz-g1` … `g10`, Post-v3 W0–W32, vitrin deploy | **100%** |
| **B — Ürün (masaüstü + motor, sertifika/mağaza hariç)** | Gerçek kullanıcı, saha, prod entegrasyon | **~58%** |
| **C — Ürün (tüm gap planı, sertifika + mağaza dahil)** | G2 EV, G3 notarize, G7 URL, G6 apk | **~40%** |
| **D — Birleşik hedef (A×40% + B×60%)** | Kod/CI ağırlıklı program görünümü | **~75%** |

> **Yorum:** Kod tarafında gap fazları **kapandı**; eksiklik ağırlığı **ürün olgunluğu** (UX saha, kurumsal oturum, motor ileri, ertelenen dış bağımlılıklar).

---

## 2) Faz faz tablo (G1–G10)

| Faz | Repo / CI | Ürün | Birleşik ≈ | Tamamlanan (özet) | Eksik / sıradaki (sertifika/mağaza hariç öncelik) |
|-----|-----------|------|------------|-------------------|--------------------------------------------------|
| **G1** Zip P0 + v1 | 100% | 92% | **96%** | Windows resmi P0×3 ✅, v1 gate ✅, macOS KPI vitrin ✅ | Linux CI proxy uyarıları; macOS dmg dağıtım kanalı vitrin |
| **G2** Windows imza | 100% | 72%* | **83%** | Dev Authenticode CI ✅, NSIS imza ✅, vitrin PNG ✅ | *EV/OV SmartScreen (ertelendi); Arc/Zip UX polish (G4) |
| **G3** macOS | 95% | 45%* | **65%** | W14 workflow ✅, W34 resmi KPI ✅ | *Notarize + geçerli `.p12` (ertelendi); dmg CI secret düzeltme |
| **G4** Masaüstü olgunluk | 75% | 48% | **58%** | Ctrl+F arama, kısayollar (?), önizleme, ZIP/.wlrt tek dosya | [Dış kullanıcı checklist](./G4_EXTERNAL_USER_SESSION_CHECKLIST.md); Zip migration saha |
| **G5** Kurumsal | 80% | 15% | **41%** | ADMX/GPO ZIP, kurumsal gate, QA şablonları | Banka/sendika **saha signoff**; gerçek AD GPO import kanıtı |
| **G6** Mobil | 100% | 10% | **37%** | W26 API, W36 gate, `mobile-inspect` | Native **AAB** kabuk (Play ertelendi) |
| **G7** Mağaza | 100% | 5%* | **43%** | W27/W32, Data safety doc ✅ | *Play internal URL (ertelendi); strateji: [G7_DESKTOP_FIRST](./G7_DESKTOP_FIRST_PLAY_INTERNAL.md) |
| **G8** Lerta | 85% | 25% | **49%** | W28–W31 staging, API JSON vitrin | Prod `LERTA_API_BEARER`, canlı submit IT onayı |
| **G9** Motor ileri | 100% | 42% | **58%** | `extract_one` ZIP+.wlrt, önizleme API, Arc UI | Solid/dedup, kısmi kurtarma |
| **G10** Marka + suite | 100% | 30% | **58%** | Tur4 policy, brand scan, suite kickoff doc | PDF/imza/sertifika **modül kodu** (Faz 3 suite) |

\* Ürün yüzdesi **ertelenen dış adım** olmadan tavanlanmış (EV/notarize/Play olmadan).

---

## 3) Bileşen bazlı (%)

| Bileşen | Tamamlanan | Kalan | ≈ % |
|---------|------------|-------|-----|
| **Ortak motor** (`lertiva-archive`) ZIP/wlrt/RAR/7z, bench, CLI | P0 bench CI, mobil okuyucu, extract_one | Solid, dedup, çok çekirdek, kısmi kurtarma | **78%** |
| **Lertiva Zip** Windows/macOS CI, Explorer, kurumsal W18 | v1 gate, P0 resmi, beta build | WinZip parity saha, notarize dmg, EV imza | **72%** |
| **Lertiva Arc** Tauri UI, RAR, geçiş sihirbazı, G4 arama/kısayol/önizleme | İmzalı Windows CI, audit | Tree görünüm, RAR tek dosya, perf vs WinRAR | **68%** |
| **Vitrin** winlrt.com.tr, benchmark kartları, pilot dalga | Deploy otomasyon, G2–G10 JSON | Canlı müşteri hikâyeleri, gerçek KPI güncelleme ritmi | **88%** |
| **Kurumsal** GPO/ADMX, Argon2, denetim | Paket + doküman | Saha pilot imzalı signoff | **45%** |
| **Mobil** W26 | API + gate | Uygulama binary | **20%** |
| **Lerta entegrasyon** staging | W31 JSON, dry-run | Prod bearer + 1 canlı kayıt | **35%** |
| **Suite vizyon** (PDF/imza/Lerta F8) | Kickoff + roadmap | Ürün kodu | **15%** |

---

## 4) Gate JSON anlık durum (`all_pass`)

| Dosya | all_pass | Not |
|-------|----------|-----|
| `lertiva-zip-p0-gate.json` | ✅ | Windows resmi KPI |
| `lertiva-zip-v1-gate.json` | ✅ | v1.0 GA repo kapısı |
| `lertiva-windows-signing-gate.json` | ✅ | Dev imza + vitrin PNG |
| `lertiva-ev-ov-signing-gate.json` | ❌ | EV/OV flag yok (ertelendi) |
| `lertiva-macos-notarize-gate.json` | ❌ | Notarize flag yok (ertelendi) |
| `lertiva-zip-store-listing-gate.json` | ✅ repo / ❌ URL | `listing_url_set` false |
| `lertiva-zip-mobile-mvp-gate.json` | ✅ | Native app yok |
| `lertiva-arc-motor-advanced-gate.json` | ✅ | MVP; ileri R&D açık |
| `lertiva-brand-suite-gate.json` | ✅ | PDF modülü yok |
| `lertiva-desktop-maturity-gate.json` | ✅ repo MVP | Dış kullanıcı oturumu insan |

Doğrulama: `bash scripts/verify-faz-all.sh` · Mac: `bash scripts/test-g2-g3-scenarios.sh`

---

## 5) Ertelenen dış bağımlılıklar (bilinçli)

| Madde | Faz | Ne zaman |
|-------|-----|----------|
| EV/OV kod imzalama PFX | G2 | CA siparişi sonrası |
| Apple Developer ID + notarize | G3 | `.p12` + 6 secret |
| Google Play internal | G7 | AAB + Console (masaüstü sonrası) |
| App Store / TestFlight | G6/G7 | Play sonrası |
| Lerta prod API | G8 | IT + KVKK onayı |

Runbook: [G2_G3_SIRALI_RUNBOOK.md](./G2_G3_SIRALI_RUNBOOK.md) · Mağaza: [G7_DESKTOP_FIRST_PLAY_INTERNAL.md](./G7_DESKTOP_FIRST_PLAY_INTERNAL.md)

---

## 6) Sıradaki faz planı (sertifika / mağaza **yok** — kod & olgunluk)

Önerilen agent + siz sırası:

| Sıra | Faz | Hedef ≈ | Yapılacaklar (kod / repo / QA) |
|------|-----|---------|--------------------------------|
| 1 | **G4** | 58%→72% | Dış kullanıcı oturumu checklist; perf regresyon; tree görünüm (opsiyonel) |
| 2 | **G9** | 55%→70% | ZIP tek dosya extract; Arc önizleme (metin/küçük dosya); bench satırı genişletme |
| 3 | **G5** | 41%→55% | Kurumsal pilot runbook + boş signoff şablonu doldurma rehberi (saha sizde) |
| 4 | **G8** | 49%→65% | Lerta staging testleri, prod config şablonu (secret yok), audit log dokümantasyonu |
| 3 | **G1 polish** | 96%→98% | macOS dmg release link vitrin; bench import otomasyonu dokümantasyon |
| — | G2/G3/G7 | — | **Sertifika / Play hazır olunca** runbook A/B |

Her faz sonu: ilgili `verify-faz-gN.sh` → commit → `agent-deploy-vitrin.sh` → bu raporu güncelle.

---

## 7) WINLRTRAR plan — açık maddeler (motor/UI)

Repo checklist ([WINLRTRAR_PLAN.md](../WINLRTRAR_PLAN.md)) — **tamamlanma ≈ %35** (açık maddeler çok):

- ✅ Kısmen: profiller, iş kuyruğu UI, şifreli wlrt, RAR okuma, G9 tek dosya `.wlrt`
- ❌ Çok çekirdek pipeline, solid/dedup, kısmi kurtarma, tree+arama, MSI/MSIX, Lerta eki profilleri

---

## 8) Tek komut özeti

```bash
bash scripts/verify-faz-all.sh          # Repo G1–G10
bash scripts/evaluate-gap-status-report.sh
bash scripts/print-g2-g3-product-progress.sh   # Mac
```

**Sonuç cümlesi:** Repo gap **%100**; program **~%75**; vitrin panosu: [/benchmark/gap/](/benchmark/gap/). Sıradaki: **G4 saha oturumu → G9 → G5 → G8** (`bash scripts/gap-phase-finish.sh 4 --deploy`).
