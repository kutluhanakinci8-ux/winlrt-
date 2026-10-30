# Parola ve şifreleme (Faz 3 beta)

## Açma

| Format | Parola |
|--------|--------|
| RAR / RAR5 | Evet (UI / CLI `--password`) |
| 7z | Evet; parolalı dosyada `7z` CLI yedek |
| ZIP | ZipCrypto veya AES (okuma) |
| `.wlrt` v1 | AES-256-GCM + PBKDF2 (salt arşiv başlığında) |
| `.wlrt` v2 | AES-256-GCM + **Argon2id** (yeni oluşturma) |

Explorer «buraya aç» parolasız; parolalı arşivlerde uygulamadan parola girin.

## Oluşturma

| Format | Şifreleme |
|--------|-----------|
| ZIP | **AES-256** (WinZip AE-2, `zip` 2.x) |
| `.wlrt` | v2 şifreli arşiv (min. 8 karakter, Argon2id) |

Explorer:

- `--secure` → UI’da «Arşivi parola ile şifrele» açık (klasör / çoklu seçim menüleri).

CLI: `winlrt-archive create … --password '…'`

Tauri: «Arşivi parola ile şifrele» + güç göstergesi.

## Explorer parola sihirbazı

`--secure` → modal (parola + tekrar + profil). Bkz. [WINLRTRAR_CORPORATE_POLICY.md](./WINLRTRAR_CORPORATE_POLICY.md).

## Sonraki

Yerel Win32 parola diyalogu (opsiyonel), GPO ile politika (Faz 4).
