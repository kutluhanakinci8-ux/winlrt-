# RAR / RAR5 okuma — Faz 2 (hukuk + teknik)

winlrtRAR **RAR üretmez**; geçiş için **yalnızca açma / listeleme / test**.

## Hukuk / lisans

| Kaynak | Not |
|--------|-----|
| [WinRAR UnRAR kaynak lisansı](https://www.rarlab.com/rar_add.htm) | UnRAR kaynak kodu; türev eser koşulları — dağıtım öncesi hukuk onayı |
| winlrt `.wlrt` yazma | Tamamen yerli; RAR formatına yazma **yok** |

**Kapı (Faz 2):** UnRAR (veya eşdeğer) kullanım şartları imzalı/onaylı → `crates/winlrt-archive/src/rar.rs` implementasyonu.

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

- [ ] RAR4 küçük fixture (lisanssız sentetik veya üretilmiş test arşivi)
- [ ] RAR5 parola / parolasız
- [ ] Bozuk arşiv hata mesajı (TR)
