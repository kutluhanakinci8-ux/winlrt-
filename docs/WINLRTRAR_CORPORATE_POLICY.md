# Lertiva Arc — kurumsal parola politikası (Faz 4 beta)

## Politikalar

| ID | Kullanım | Min. uzunluk | Büyük / küçük / rakam / sembol |
|----|----------|--------------|--------------------------------|
| `consumer` | Uygulama içi oluşturma (varsayılan) | 8 | Hayır |
| `corporate` | Explorer `--secure` menüleri | 12 | Evet (hepsi) |

## Ortam değişkeni

```bash
# Tüm oluşturma yollarında kurumsal politika
export WINLRT_PASSWORD_POLICY=corporate
```

Windows (kalıcı pilot):

```powershell
setx WINLRT_PASSWORD_POLICY corporate
```

Explorer **şifreli sıkıştır** menüsü, env tanımlı değilse otomatik **kurumsal** politika uygular.

## Explorer parola sihirbazı

`--secure` ile başlatıldığında modal açılır: parola + tekrar + profil → **Oluştur** (`.wlrt` v2 / ZIP AES).

Kaynak: `secure-create-wizard` event · `startup_secure_wizard` komutu.

## Windows yerel parola penceresi

Explorer `--secure`: önce **Win32 parola diyalogu**; iptal → web sihirbazı.  
Kapatma: `WINLRT_DISABLE_NATIVE_PASSWORD=1`

## Modern şifreleme kapısı (Faz 3)

`RequireModernCrypto=1` (GPO / kayıt) veya `WINLRT_REQUIRE_MODERN_CRYPTO=1` — **kurumsal** politika ile birlikte:

- Şifreli `.wlrt` → v2 **Argon2id**
- Şifreli ZIP → **AES-256** (ZipCrypto yok)

Tauri: `enterprise_policy_info.require_modern_crypto` · ADMX: `deploy/gpo/lertiva-arc.admx`

## Denetim

Bkz. [WINLRTRAR_AUDIT.md](./WINLRTRAR_AUDIT.md).

## Sonraki (Faz 4)

GPO/MDM ile politika dağıtımı, merkezi log toplama.
