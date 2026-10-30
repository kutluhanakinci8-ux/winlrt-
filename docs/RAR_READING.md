# RAR / RAR5 okuma — Faz 2 (hukuk + teknik)

winlrtRAR **RAR üretmez**; geçiş için **yalnızca açma / listeleme / test**.

## Hukuk / lisans

| Kaynak | Not |
|--------|-----|
| [WinRAR UnRAR kaynak lisansı](https://www.rarlab.com/rar_add.htm) | UnRAR kaynak kodu; türev eser koşulları — dağıtım öncesi hukuk onayı |
| winlrt `.wlrt` yazma | Tamamen yerli; RAR formatına yazma **yok** |

**Uygulama (Faz 2 beta):** `rar` crate (MIT, RAR5 odaklı) + isteğe bağlı sistem `unrar lb` büyük arşiv listesi için.

## Teknik seçenekler (değerlendirme)

| Yol | Artı | Eksi |
|-----|------|------|
| **UnRAR C++ statik link** (resmi kaynak) | Uyumluluk yüksek | FFI, lisans, platform build |
| **`unrar` sistem binary** (`Command`) | Hızlı prototip | Kullanıcıda binary gerekir, kurumsal zor |
| **Rust `unrar` / wrapper crate** | Entegrasyon kolay | Bakım, lisans, RAR5 edge case |

**Öneri:** Faz 2 beta → resmi **UnRAR** kaynak + ince `rar.rs` FFI katmanı; kurumsal pakette DLL/static tek paket.

## API (motor)

```rust
// crates/winlrt-archive/src/rar.rs — şu an stub
rar::list(archive)
rar::extract(archive, dest)
rar::test(archive)
```

CLI / Tauri: `--format rar` Faz 2 implementasyonu sonrası.

## Test planı

- [x] RAR5 fixture — `benchmarks/fixtures/rar/rar5-text.rar` (parola: `test`)
- [ ] RAR4 genişletme
- [ ] Bozuk arşiv hata mesajı (TR) — iyileştirme
