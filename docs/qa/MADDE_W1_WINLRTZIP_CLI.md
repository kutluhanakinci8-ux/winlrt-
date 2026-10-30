# Madde W1 — Lertiva Zip motor / CLI %100

## Kapsam (Faz 1 — kabuk yok)

| Çıktı | Durum |
|-------|--------|
| `winlrt-zip` binary | `crates/lertiva-archive` |
| create / extract / list / test / bench | ZIP only |
| Varsayılan create profili | `email` (e-posta eki) |
| Motor | `zip_io` + ortak `bench` |
| CI | `cargo test -p lertiva-archive` + `--bins` |

## Kullanım

```bash
cargo build -p lertiva-archive --release --bin lertiva-zip
winlrt-zip create ./klasor ./cikti.zip --profile balanced
winlrt-zip list ./cikti.zip
winlrt-zip bench --all --profile fast --json
```

Doğrulama: `bash scripts/verify-madde-w1.sh`

**Sonraki:** W2 — `apps/lertiva-zip` Tauri kabuk.
