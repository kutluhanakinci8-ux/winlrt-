# Madde W8 — Lertiva Zip AES-256 interop (I4) %100

## Senaryo I4

| Adım | Beklenen |
|------|----------|
| AES-256 ZIP oluştur | WinZip AE-2 extra field (`0x9901`) |
| Parola ile aç | Motor `list` / `extract` / `test` |
| Yanlış parola | Red |
| 7-Zip (varsa) | Harici açma uyumlu |

Parola örneği (kurumsal politika): `Winlrt-Pilot12` (test sabiti).

## Otomasyon

| Bileşen | Konum |
|---------|--------|
| Testler | `crates/lertiva-archive/tests/lertiva_zip_interop.rs` (`i4_*`) |
| Motor | `zip_io` — `AesMode::Aes256` |
| CI | `main-pipeline` — `p7zip-full` + `i4_` test filtresi |

## W8 %100

1. `bash scripts/verify-madde-w8.sh`
2. Linux CI: 7-Zip çapraz açma yeşil
