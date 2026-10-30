#!/usr/bin/env bash
# Madde W28: Lertiva Zip Lerta posta eki otomasyonu.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q prepare_lerta_mail_attachment "$ROOT/crates/lertiva-archive/src/lerta_mail.rs"
grep -q lerta_prepare_mail_attachment "$ROOT/apps/lertiva-zip/src-tauri/src/lib.rs"
test -f "$ROOT/docs/qa/MADDE_W28_WINLRTZIP_LERTA_MAIL.md"
test -f "$ROOT/docs/qa/lerta-mail-v1.schema.json"
cargo test -p lertiva-archive lerta_mail:: --quiet
bash "$ROOT/scripts/evaluate-lerta-mail-sample.sh"
test -f "$ROOT/apps/web/public/benchmark/samples/winlrtzip-lerta-mail-v1.json"
cargo test -p lertiva-archive --test winlrtzip_interop w28_ --quiet
cargo check -p lertiva-zip --quiet
echo "OK: Madde W28 — Lertiva Zip Lerta posta eki"
