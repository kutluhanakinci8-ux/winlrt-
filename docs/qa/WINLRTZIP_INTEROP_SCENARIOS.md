# winlrtZIP — ZIP interop test senaryoları (Faz 0)

Motor sprint (Faz 1) öncesi kabul listesi. Her senaryo: **oluştur → aç** (Windows + isteğe macOS).

| ID | Senaryo | Beklenen |
|----|---------|----------|
| I1 | Explorer ile oluşturulan ZIP → winlrt aç | Tüm dosyalar, yol doğru |
| I2 | winlrt ZIP → Explorer “Tümünü çıkar” | Uyarı yok |
| I3 | ZIP64 (>4 GiB simülasyon veya büyük dosya) | **Otomatik** — [MADDE_W10](./MADDE_W10_WINLRTZIP_ZIP64.md) |
| I4 | AES-256 şifreli ZIP (WinZip uyumlu) | **Otomatik** — [MADDE_W8](./MADDE_W8_WINLRTZIP_AES_INTEROP.md) |
| I5 | Türkçe dosya adları (UTF-8) | **Otomatik** — [MADDE_W9](./MADDE_W9_WINLRTZIP_UTF8.md) |
| I6 | E-posta eki profili (~10 MB hedef) | **Otomatik** — [MADDE_W7](./MADDE_W7_WINLRTZIP_EMAIL.md) |
| I7 | Bozuk CRC (kısmi) | Kontrollü hata mesajı |

Otomasyon hedefi: Faz 1’de `cargo test` + isteğe Windows script.
