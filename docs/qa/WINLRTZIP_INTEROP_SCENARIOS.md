# Lertiva Zip — ZIP interop test senaryoları (Faz 0)

Motor sprint (Faz 1) öncesi kabul listesi. Her senaryo: **oluştur → aç** (Windows + isteğe macOS).

| ID | Senaryo | Beklenen |
|----|---------|----------|
| I1 | Explorer ile oluşturulan ZIP → winlrt aç | **Proxy CI** — [MADDE_W12](./MADDE_W12_WINLRTZIP_EXPLORER_I1_I2.md) · saha: Explorer sıkıştır |
| I2 | winlrt ZIP → Explorer “Tümünü çıkar” | **Proxy CI** — [MADDE_W12](./MADDE_W12_WINLRTZIP_EXPLORER_I1_I2.md) · saha: Tümünü çıkar |
| I3 | ZIP64 (>4 GiB simülasyon veya büyük dosya) | **Otomatik** — [MADDE_W10](./MADDE_W10_WINLRTZIP_ZIP64.md) |
| I4 | AES-256 şifreli ZIP (WinZip uyumlu) | **Otomatik** — [MADDE_W8](./MADDE_W8_WINLRTZIP_AES_INTEROP.md) |
| I5 | Türkçe dosya adları (UTF-8) | **Otomatik** — [MADDE_W9](./MADDE_W9_WINLRTZIP_UTF8.md) |
| I6 | E-posta eki profili (~10 MB hedef) | **Otomatik** — [MADDE_W7](./MADDE_W7_WINLRTZIP_EMAIL.md) |
| I7 | Bozuk CRC (kısmi) | **Otomatik** — [MADDE_W11](./MADDE_W11_WINLRTZIP_CRC_I7.md) |

Otomasyon hedefi: Faz 1’de `cargo test` + isteğe Windows script.
