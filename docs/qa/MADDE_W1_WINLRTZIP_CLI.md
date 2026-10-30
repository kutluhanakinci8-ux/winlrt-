# Madde W1 — winlrtZIP motor / CLI %100

## Kapsam (Faz 1 — kabuk yok)

| Çıktı | Durum |
|-------|--------|
| `winlrt-zip` binary | `crates/winlrt-archive` |
| create / extract / list / test / bench | ZIP only |
| Varsayılan create profili | `email` (e-posta eki) |
| Motor | `zip_io` + ortak `bench` |
| CI | `cargo test -p winlrt-archive` + `--bins` |

## Kullanım

```bash
cargo build -p winlrt-archive --release --bin winlrt-zip
winlrt-zip create ./klasor ./cikti.zip --profile balanced
winlrt-zip list ./cikti.zip
winlrt-zip bench --all --profile fast --json
```

Doğrulama: `bash scripts/verify-madde-w1.sh`

**Sonraki:** W2 — `apps/winlrt-zip` Tauri kabuk.
