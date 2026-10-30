#!/usr/bin/env bash
# Madde W7: winlrtZIP e-posta eki profili (I6).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
grep -q EMAIL_ATTACHMENT_MAX_ZIP_BYTES "$ROOT/crates/winlrt-archive/src/profile.rs"
test -f "$ROOT/crates/winlrt-archive/tests/winlrtzip_interop.rs"
cargo test -p winlrt-archive --test winlrtzip_interop i6_email_profile_small --quiet
bash "$ROOT/benchmarks/fixtures/mixed/prepare.sh"
cargo test -p winlrt-archive --test winlrtzip_interop i6_email_profile_mixed -- --ignored --quiet
echo "OK: Madde W7 - winlrtZIP e-posta profili (I6)"
