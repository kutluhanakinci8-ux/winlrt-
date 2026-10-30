# winlrt-archive

winlrtRAR / winlrtZIP ortak Rust motoru.

## Geliştirme

```bash
cargo test -p winlrt-archive
cargo run -p winlrt-archive -- --help
cargo run -p winlrt-archive -- bench --fixture benchmarks/fixtures/text --json
```

## Faz 1 durumu

- ZIP: `list`, `extract`, `create`, `test` (profil: Fast → stored)
- `.wlrt` v0: zstd + çok çekirdek sıkıştırma — [WLRT_FORMAT_V0.md](../../docs/WLRT_FORMAT_V0.md)
- CLI: `--json`, `bench`
- RAR açma: **Faz 2**

Plan: [docs/WINLRTRAR_PLAN.md](../../docs/WINLRTRAR_PLAN.md)
