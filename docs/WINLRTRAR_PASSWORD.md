# Parola ve şifreleme (Faz 3 beta)

## Açma

| Format | Parola |
|--------|--------|
| RAR / RAR5 | Evet (UI / CLI `--password`) |
| 7z | Evet; parolalı dosyada `7z` CLI yedek |
| ZIP | ZipCrypto veya AES (okuma) |
| `.wlrt` v1 | AES-256-GCM + PBKDF2 (salt arşiv başlığında) |

Explorer «buraya aç» parolasız; parolalı arşivlerde uygulamadan parola girin.

## Oluşturma

| Format | Şifreleme |
|--------|-----------|
| ZIP | ZipCrypto (beta, `zip` crate unstable) — kurumsal hedef: WinZip AE-2 |
| `.wlrt` | v1 şifreli arşiv (min. 8 karakter) |

CLI: `winlrt-archive create … --password '…'`

Tauri: «Arşivi parola ile şifrele» + güç göstergesi.

## Sonraki

Argon2id türetme, ZIP AES yazma, kurumsal parola politikası (Faz 4).
