# winlrtZIP — ZIP interop test senaryoları (Faz 0)

Motor sprint (Faz 1) öncesi kabul listesi. Her senaryo: **oluştur → aç** (Windows + isteğe macOS).

| ID | Senaryo | Beklenen |
|----|---------|----------|
| I1 | Explorer ile oluşturulan ZIP → winlrt aç | Tüm dosyalar, yol doğru |
| I2 | winlrt ZIP → Explorer “Tümünü çıkar” | Uyarı yok |
| I3 | ZIP64 (>4 GiB simülasyon veya büyük dosya) | Okuma/yazma |
| I4 | AES-256 şifreli ZIP (WinZip uyumlu) | Parola ile açma |
| I5 | Türkçe dosya adları (UTF-8) | Kodlama bozulmaz |
| I6 | E-posta eki profili (~10 MB hedef) | **Otomatik** — [MADDE_W7](./MADDE_W7_WINLRTZIP_EMAIL.md) |
| I7 | Bozuk CRC (kısmi) | Kontrollü hata mesajı |

Otomasyon hedefi: Faz 1’de `cargo test` + isteğe Windows script.
