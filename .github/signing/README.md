# Lertiva geliştirme kod imzası (self-signed)

Gerçek **EV/OV Authenticode** gelene kadar aynı CI hattında (`TAURI_SIGNING_*`) kullanılan **kurumsal geliştirme** sertifikası.

| Dosya | Repoda? |
|-------|---------|
| `lertiva-dev-codesign.cer` | Evet — yalnızca **public** |
| `*.pfx`, `*.key.pem` | **Hayır** — `.signing-local/` (gitignore) |

## Üretim

**Linux / macOS / agent:**

```bash
export LERTIVA_DEV_CODESIGN_PASSWORD='…'
bash scripts/generate-lertiva-dev-codesign.sh
bash scripts/push-lertiva-dev-signing-secrets.sh   # gh yetkisi gerekir
```

**Windows (PowerShell):**

```powershell
.\scripts\windows\new-lertiva-dev-codesign-cert.ps1
```

## Beklenti

- `signtool verify /pa` → **geçer** (imzalı binary).
- **SmartScreen** → bilinmeyen yayıncı uyarısı **devam eder** (EV/OV değil).
- Gerçek sertifika gelince aynı secret adlarına PFX değiştirilir; public `.cer` güncellenir.

## Güven dağıtımı (isteğe bağlı)

Kurumsal test PC’lerde public `.cer` → `LocalMachine\Root` + `TrustedPublisher` (IT). Bkz. `generate-lertiva-dev-codesign.sh` çıktısı.
