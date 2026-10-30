# lertiva-archive

Lertiva Arc / Lertiva Zip ortak Rust motoru.

## winlrt-zip CLI (Faz 1)

```bash
cargo run -p lertiva-archive --bin lertiva-zip -- create ./src ./out.zip --profile email
```

## Geliştirme

```bash
cargo test -p lertiva-archive
cargo run -p lertiva-archive -- --help
cargo run -p lertiva-archive -- bench --fixture benchmarks/fixtures/text --json
```

## Faz 1 durumu

- ZIP: `list`, `extract`, `create`, `test` (profil: Fast → stored)
- `.wlrt` v0: zstd + çok çekirdek sıkıştırma — [WLRT_FORMAT_V0.md](../../docs/WLRT_FORMAT_V0.md)
- CLI: `--json`, `bench`
- RAR açma: **Faz 2**

Plan: [docs/WINLRTRAR_PLAN.md](../../docs/WINLRTRAR_PLAN.md)
