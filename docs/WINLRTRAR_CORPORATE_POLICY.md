# winlrtRAR — kurumsal parola politikası (Faz 4 beta)

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

## Sonraki (Faz 4)

GPO/MDM ile politika dağıtımı, denetim günlüğü (parola denemesi sayısı — parola metni **loglanmaz**).
