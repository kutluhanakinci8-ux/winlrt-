# G2 → G3 sıralı runbook (masaüstü imza)

**Sıra:** Önce **G2 Windows EV/OV**, bitince **G3 macOS notarize**. Play mağaza (G7) daha sonra — [G7_DESKTOP_FIRST_PLAY_INTERNAL.md](./G7_DESKTOP_FIRST_PLAY_INTERNAL.md).

Durum özeti: `bash scripts/print-g2-g3-product-progress.sh`

---

## Bölüm A — G2 EV/OV Authenticode (Windows)

### A0 — Bugünkü taban (tamamlandı sayılır)

| # | Madde | Kontrol |
|---|--------|---------|
| A0.1 | Dev PFX CI imzası | `bash scripts/verify-madde-h-signed.sh` |
| A0.2 | Zip + Arc workflow yeşil | Actions → `lertiva-zip-windows.yml`, `lertiva-arc-windows.yml` |
| A0.3 | Vitrin SmartScreen PNG | `incoming/smartscreen-lertiva.png` |

JSON: `lertiva-ev-ov-signing-gate.json` → `dev_authenticode_ci: true`, `ev_ov_pfx_production: false` (flag yokken).

---

### A1 — Kurumsal sertifika (insan / satın alma)

| # | Adım | Not |
|---|------|-----|
| A1.1 | **EV** (tercih) veya **OV** kod imzalama sertifikası sipariş et | SmartScreen güveni EV ile daha iyi |
| A1.2 | CA doğrulama (KOBİ: ticaret sicil, yetkili imza) | 1–10 iş günü CA’ya göre değişir |
| A1.3 | PFX (.pfx / .p12) + export parolası al | Private key PFX içinde kalsın |
| A1.4 | Yayıncı adını vitrin ile hizala | Örn. şirket unvanı = SmartScreen’de görünen ad |

Hukuk / lisans: [WINLRTRAR_CODE_SIGNING.md](../WINLRTRAR_CODE_SIGNING.md)

---

### A2 — PFX → GitHub Secrets

| # | Adım | Komut / yer |
|---|------|-------------|
| A2.1 | PFX’i **base64 tek satır** yap (Mac/Linux) | `base64 -w0 kurumsal.pfx \| pbcopy` veya `base64 -i kurumsal.pfx \| tr -d '\n'` |
| A2.2 | [Actions secrets](https://github.com/kutluhanakinci8-ux/winlrt-/settings/secrets/actions) | |
| A2.3 | **`WINDOWS_CERTIFICATE`** | A2.1 çıktısı (tam satır) |
| A2.4 | **`WINDOWS_CERTIFICATE_PASSWORD`** | PFX export parolası |
| A2.5 | (Opsiyonel) Eski dev secret’ları **silme** | EV geçince dev’i yedek tut; prod secret’lar öncelikli |

Workflow fallback: `WINDOWS_CERTIFICATE` yoksa `TAURI_SIGNING_PRIVATE_KEY` (dev) kullanılır. **EV/OV için A2.3–A2.4 zorunlu.**

Detay: [GITHUB_TAURI_SECRETS_KURULUM.md](./GITHUB_TAURI_SECRETS_KURULUM.md)

---

### A3 — CI doğrulama

| # | Adım | Beklenen |
|---|------|----------|
| A3.1 | Actions → **Lertiva Zip Windows beta build** → Run workflow | |
| A3.2 | Actions → **Lertiva Arc Windows** (aynı repo workflow adı) → Run workflow | |
| A3.3 | Log: `sign-windows-nsis-bundle.ps1` | `Successfully signed` |
| A3.4 | Log: `Verify Authenticode` | `signtool verify /pa` **Passed** |
| A3.5 | Artifact indir → yerel Windows’ta SmartScreen | EV/OV: “Bilinmeyen yayıncı” azalır veya kaybolur |
| A3.6 | (Opsiyonel) Yeni vitrin PNG | Gerçek SmartScreen → `incoming/smartscreen-lertiva.png` |

---

### A4 — Repo “ürün kapısı” (G2 tamam)

| # | Adım | |
|---|------|--|
| A4.1 | Flag oluştur | `echo ev_ov > apps/web/public/benchmark/samples/lertiva-ev-ov-configured.flag` |
| A4.2 | Gate yenile | `bash scripts/evaluate-lertiva-ev-ov-signing-gate.sh` |
| A4.3 | Kontrol | `jq . apps/web/public/benchmark/samples/lertiva-ev-ov-signing-gate.json` → **`all_pass: true`** |
| A4.4 | Commit + push | flag + güncel JSON |
| A4.5 | Vitrin deploy | `bash scripts/agent-deploy-vitrin.sh` veya main CI |

**G2 bitmeden G3’e geçmeyin** (aynı sprintte paralel secret hazırlığı OK).

---

## Bölüm B — G3 macOS notarize (Zip dmg)

Önkoşul: Apple Developer Program (**Team ID**), **Developer ID Application** sertifikası.

### B1 — Apple tarafı

| # | Adım | Not |
|---|------|-----|
| B1.1 | [Apple Developer](https://developer.apple.com) üyelik | Yıllık |
| B1.2 | **Certificates** → **Developer ID Application** oluştur | Mac’te Keychain Access veya Xcode |
| B1.3 | `.p12` export + parola | CI için |
| B1.4 | [App-specific password](https://appleid.apple.com) | Notary aracı için |
| B1.5 | **Team ID** kopyala | Membership details |

---

### B2 — GitHub Secrets (6 adet)

| Secret | Değer |
|--------|--------|
| `APPLE_CERTIFICATE` | .p12 base64 (tek satır) |
| `APPLE_CERTIFICATE_PASSWORD` | .p12 parolası |
| `APPLE_SIGNING_IDENTITY` | Tam ad, örn. `Developer ID Application: Sirket Adi (TEAMID)` |
| `APPLE_ID` | Apple ID e-posta |
| `APPLE_PASSWORD` | App-specific password |
| `APPLE_TEAM_ID` | 10 karakter Team ID |

Workflow: [.github/workflows/lertiva-zip-macos.yml](../../.github/workflows/lertiva-zip-macos.yml) — build adımında env olarak geçer; Tauri imza/notarize bu değişkenleri kullanır.

---

### B3 — CI doğrulama

| # | Adım | Beklenen |
|---|------|----------|
| B3.1 | Actions → **Lertiva Zip macOS beta build** → Run workflow | |
| B3.2 | Adım **Notarize / imza durumu** | `present=true` (APPLE_CERTIFICATE dolu) |
| B3.3 | dmg artifact indir | macOS: `spctl -a -vv -t install Lertiva*.dmg` → **accepted** / notarized |
| B3.4 | (Opsiyonel) Release flag | `.github/flags/trigger-zip-macos-release` push |

KPI job (`macos-zip-kpi`) notarize **gerektirmez** — W34 ayrı.

---

### B4 — Repo “ürün kapısı” (G3 tamam)

| # | Adım | |
|---|------|--|
| B4.1 | Flag | `echo notarize > apps/web/public/benchmark/samples/lertiva-apple-notarize-configured.flag` |
| B4.2 | Gate | `bash scripts/evaluate-lertiva-macos-notarize-gate.sh` |
| B4.3 | Kontrol | `lertiva-macos-notarize-gate.json` → **`all_pass: true`** |
| B4.4 | Commit + deploy | flag + JSON |

---

## Mac geliştirici kurulumu

```bash
cd ~/winlrt-
# quickcheck/evaluate gate JSON dosyalarini degistirir — pull oncesi:
bash scripts/mac-sync-repo-main.sh
brew install jq
bash scripts/check-g2-g3-prerequisites.sh
bash scripts/mac-g2-g3-quickcheck.sh
bash scripts/test-g2-g3-scenarios.sh
```

`git pull` hata verirse: `git checkout -- apps/web/public/benchmark/samples/lertiva-*-signing-gate.json apps/web/public/benchmark/samples/lertiva-macos-notarize-gate.json` sonra tekrar pull.

**Zorunlu:** `jq` (`brew install jq`).  
**Opsiyonel:** `gh` — GitHub Actions logları (G2-S3). **macOS 11 (Big Sur):** `brew install gh` çoğu zaman **go** yüzünden düşer; `gh auth login` çalıştırmayın (gh yoksa). Alternatif:

```bash
# GitHub CLI — eski sürüm .pkg (Monterey+ şart değil, sürüme bakın)
# https://github.com/cli/cli/releases — örn. gh_*_macOS_amd64.pkg
```

`gh` olmadan: `verify-madde-h-signing-repo-local.sh` + ⚠️ CI atlanır; imza hattı repo dosyaları yine kontrol edilir.

## Senaryo testleri (agent)

```bash
bash scripts/test-g2-g3-scenarios.sh
```

Beklenen (dev PFX, EV/OV henüz yok): G2-S5 `ev_ov` false, G2-S3 CI yeşil, G3-S2 notarize false.  
**A3 workflow dispatch:** Cloud agent `403` — siz Actions → Run workflow.

## Hızlı komutlar

```bash
# İlerleme tablosu
bash scripts/print-g2-g3-product-progress.sh

# Sadece G2
bash scripts/evaluate-lertiva-ev-ov-signing-gate.sh

# Sadece G3
bash scripts/evaluate-lertiva-macos-notarize-gate.sh

# Ürün gap özeti (G2/G3 dahil)
bash scripts/verify-product-gap-all.sh
```

---

## Sıradaki faz (G2+G3 sonrası)

1. G4/G5 saha QA (isteğe bağlı paralel)
2. G8 Lerta prod bearer
3. Android AAB + **Play internal** (G7 URL)
