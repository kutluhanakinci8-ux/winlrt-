#!/usr/bin/env bash
# Madde W15: WinZip gecis sihirbazi + macOS KPI vitrin ornekleri.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q winzip_migration_guide "$ROOT/apps/winlrt-zip/src/App.tsx"
grep -q WinZip "$ROOT/apps/winlrt-zip/src/App.tsx"
test -f "$ROOT/apps/winlrt-zip/src-tauri/src/migration.rs"
test -f "$ROOT/docs/qa/MADDE_W15_WINLRTZIP_MIGRATION_MACOS_KPI.md"
test -f "$ROOT/apps/web/public/benchmark/samples/winlrtzip-macos-balanced-kpi.json"
test -f "$ROOT/apps/web/public/benchmark/samples/winlrtzip-macos-fast-kpi.json"
grep -q winlrtzip-macos "$ROOT/apps/web/public/benchmark/index.html"

cargo test -p winlrt-zip migration_guide --quiet
cargo check -p winlrt-zip --quiet

echo "OK: Madde W15 — WinZip gecis + macOS KPI vitrin"
