#!/usr/bin/env bash
# Madde W29: winlrtZIP Lerta üretim API smoke.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q submit_mail_attachment_metadata "$ROOT/crates/winlrt-archive/src/lerta_api.rs"
grep -q lerta_api_status "$ROOT/apps/winlrt-zip/src-tauri/src/lib.rs"
test -f "$ROOT/docs/qa/MADDE_W29_WINLRTZIP_LERTA_API.md"
test -f "$ROOT/docs/qa/lerta-api-v1.schema.json"
cargo test -p winlrt-archive lerta_api:: --quiet
cargo test -p winlrt-archive --test winlrtzip_interop w29_ --quiet
bash "$ROOT/scripts/evaluate-lerta-api-sample.sh"
test -f "$ROOT/apps/web/public/benchmark/samples/winlrtzip-lerta-api-v1.json"
echo "OK: Madde W29 — winlrtZIP Lerta API"
