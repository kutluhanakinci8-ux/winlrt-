# Lertiva Arc — Explorer / NSIS pilot QA (Faz 3)

Windows beta kurulumundan sonra **sırayla** işaretleyin. Kaynak menüler: [WINLRTRAR_EXPLORER_MENU.md](./WINLRTRAR_EXPLORER_MENU.md).

## Kurulum

- [ ] NSIS `.exe` veya `.msi` kuruldu (`ProductVersion` kayıt defterinde)
- [ ] Kaldır-yeniden-kur ile menüler çiftlenmedi

## Arşiv uzantıları (zip / wlrt / rar / 7z)

- [ ] **Lertiva Arc ile aç** — uygulama açılır, liste görünür
- [ ] **buraya aç** — aynı klasörde `*_extract` veya hedef klasör oluşur
- [ ] Parolalı `.rar` / `.7z`: buraya aç → uygulama içinden parola (bilinen sınırlama)

## Klasör

- [ ] **sıkıştır** — önerilen `.wlrt` yolu mantıklı
- [ ] **hızlı .wlrt** — komşu dosya oluşur
- [ ] **şifreli sıkıştır** — Win32 parola veya web sihirbazı; kurumsal politika

## Çoklu dosya (`*`)

- [ ] Aynı klasör: arşiv adı **ilk dosya stem** (WinRAR)
- [ ] Farklı klasörler: **ilk seçimin klasörü + stem**
- [ ] **seçilenler şifreli** — audit satırı (`secure_password_prompt`) oluşur

## Kurumsal (isteğe bağlı pilot)

- [ ] GPO / `.reg`: `PasswordPolicy=corporate`, `RequireModernCrypto=1`
- [ ] Zayıf parola reddedilir; güçlü parola ile şifreli `.wlrt` / ZIP AES

## Temizlik

- [ ] Program kaldırma → Explorer menüleri silindi (`NSIS_HOOK_POSTUNINSTALL`)

İlgili: [WINLRTRAR_PRODUCTION_CHECKLIST.md](./WINLRTRAR_PRODUCTION_CHECKLIST.md)
