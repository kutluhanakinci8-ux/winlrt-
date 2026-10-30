#!/usr/bin/env bash
# Madde W30: winlrtZIP Lerta mesaj eki (.wlrt / AES-ZIP metadata).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q prepare_lerta_message_attachment "$ROOT/crates/winlrt-archive/src/lerta_message.rs"
grep -q lerta_prepare_message_attachment "$ROOT/apps/winlrt-zip/src-tauri/src/lib.rs"
grep -q submit_message_attachment_metadata "$ROOT/crates/winlrt-archive/src/lerta_api.rs"
test -f "$ROOT/docs/qa/MADDE_W30_WINLRTZIP_LERTA_MESSAGE.md"
test -f "$ROOT/docs/qa/lerta-message-v1.schema.json"
cargo test -p winlrt-archive lerta_message:: --quiet
cargo test -p winlrt-archive lerta_api:: --quiet
bash "$ROOT/scripts/evaluate-lerta-message-sample.sh"
test -f "$ROOT/apps/web/public/benchmark/samples/winlrtzip-lerta-message-v1.json"
cargo test -p winlrt-archive --test winlrtzip_interop w30_ --quiet
cargo check -p winlrt-zip --quiet
echo "OK: Madde W30 — winlrtZIP Lerta mesaj eki"
