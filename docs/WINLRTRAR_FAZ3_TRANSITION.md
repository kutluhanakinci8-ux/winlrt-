# Faz 3 — Geçiş sihirbazı (taslak)

WinRAR kullanıcısının winlrtRAR’ı **varsayılan arşiv uygulaması** yapması.

## Hedef adımlar (Windows)

1. **Kurulum** — GitHub Releases’ten beta `.exe` / `.msi`; SmartScreen uyarısında “Yine de çalıştır” (imzalı kurulum Faz 3 kapısı).
2. **Dosya ilişkilendirme** — Kurulum `fileAssociations` (zip, wlrt, rar, 7z) kaydeder; gerekirse Ayarlar → Uygulamalar → Varsayılan uygulamalar.
3. **Çift tık testi** — Örnek `.zip` / `.rar` ile winlrtRAR açılır; `startup_archives` + `open-archive` olayı.
4. **WinRAR birlikte** — RAR **yazma** yok; mevcut RAR arşivleri **okuma** ile açılır; yeni arşivler `.wlrt` veya ZIP önerilir.
5. **Kısayol / Explorer** (Faz 3) — Sağ tık menüsü, masaüstü kısayolu.

## Uygulama (sıra)

- [ ] Tauri içi “Geçiş” sayfası (TR): yukarıdaki adımlar + ekran görüntüsü yer tutucuları
- [ ] Windows Registry / `SetUserFTA` araştırması (yalnızca kullanıcı onayı ile)
- [ ] Explorer kabuk uzantısı (Faz 3 ana iş)

## Kapı

Geçiş sihirbazı + Explorer menüsü canlı; Faz 2 anket kapısı korunur.
