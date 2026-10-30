#!/usr/bin/env bash
# Lertiva marka 2. tur: dosya adları, vitrin asset, benchmark JSON, GPO, script adları.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

mv_if() {
  local src="$1" dst="$2"
  if [ -e "$src" ] && [ "$src" != "$dst" ]; then
    mkdir -p "$(dirname "$dst")"
    git mv "$src" "$dst" 2>/dev/null || mv "$src" "$dst"
    echo "mv: $src → $dst"
  fi
}

# --- Benchmark samples ---
for f in "$ROOT"/apps/web/public/benchmark/samples/lertiva-zip-*; do
  [ -e "$f" ] || continue
  base=$(basename "$f")
  mv_if "$f" "$ROOT/apps/web/public/benchmark/samples/${base/lertiva-zip-/lertiva-zip-}"
done

# --- Gallery PNG ---
G="$ROOT/apps/web/public/assets/gallery"
mv_if "$G/tauri-lertiva-zip-main.png" "$G/tauri-lertiva-zip-main.png"
mv_if "$G/tauri-lertiva-arc-main.png" "$G/tauri-lertiva-arc-main.png"
mv_if "$G/installer-lertiva-zip-setup.png" "$G/installer-lertiva-zip-setup.png"
mv_if "$G/installer-lertiva-arc-setup.png" "$G/installer-lertiva-arc-setup.png"
mv_if "$G/smartscreen-lertiva.png" "$G/smartscreen-lertiva.png"

# --- Downloads corporate zips (regenerate on build; rename if present) ---
D="$ROOT/apps/web/public/downloads"
mv_if "$D/lertiva-zip-corporate-gpo.zip" "$D/lertiva-zip-corporate-gpo.zip"
mv_if "$D/lertiva-arc-corporate-gpo.zip" "$D/lertiva-arc-corporate-gpo.zip"
mv_if "$D/lertiva-zip-corporate-gpo.zip" "$D/lertiva-zip-corporate-gpo.zip"
mv_if "$D/lertiva-arc-corporate-gpo.zip" "$D/lertiva-arc-corporate-gpo.zip"

# --- GPO templates ---
mv_if "$ROOT/deploy/gpo/winlrtZIP.admx" "$ROOT/deploy/gpo/lertiva-zip.admx"
mv_if "$ROOT/deploy/gpo/winlrtRAR.admx" "$ROOT/deploy/gpo/lertiva-arc.admx"
mv_if "$ROOT/deploy/gpo/en-US/winlrtZIP.adml" "$ROOT/deploy/gpo/en-US/lertiva-zip.adml"
mv_if "$ROOT/deploy/gpo/en-US/winlrtRAR.adml" "$ROOT/deploy/gpo/en-US/lertiva-arc.adml"
mv_if "$ROOT/deploy/gpo/tr-TR/winlrtZIP.adml" "$ROOT/deploy/gpo/tr-TR/lertiva-zip.adml"
mv_if "$ROOT/deploy/gpo/tr-TR/winlrtRAR.adml" "$ROOT/deploy/gpo/tr-TR/lertiva-arc.adml"
mv_if "$ROOT/deploy/gpo/README-winlrtZIP.md" "$ROOT/deploy/gpo/README-lertiva-zip.md"

# --- Vitrin screenshot mocks ---
V="$ROOT/scripts/vitrin-screenshots"
mv_if "$V/lertiva-zip-mock.html" "$V/lertiva-zip-mock.html"
mv_if "$V/lertiva-arc-mock.html" "$V/lertiva-arc-mock.html"
mv_if "$V/lertiva-zip-installer-mock.html" "$V/lertiva-zip-installer-mock.html"
mv_if "$V/lertiva-arc-installer-mock.html" "$V/lertiva-arc-installer-mock.html"

# --- Scripts (kebab filenames) ---
rename_script() {
  local old="$1" new="$2"
  mv_if "$ROOT/scripts/$old" "$ROOT/scripts/$new"
}
rename_script "package-lertiva-zip-corporate-gpo-zip.sh" "package-lertiva-zip-corporate-gpo-zip.sh"
rename_script "lertiva-zip-corporate-policy-sample.reg" "lertiva-zip-corporate-policy-sample.reg"
rename_script "lertiva-arc-corporate-policy-sample.reg" "lertiva-arc-corporate-policy-sample.reg"
rename_script "evaluate-lertiva-zip-p0-gate.sh" "evaluate-lertiva-zip-p0-gate.sh"
rename_script "refresh-lertiva-zip-p0-gate.sh" "refresh-lertiva-zip-p0-gate.sh"
rename_script "refresh-lertiva-zip-vitrin-samples.sh" "refresh-lertiva-zip-vitrin-samples.sh"
rename_script "evaluate-lertiva-zip-store-gate.sh" "evaluate-lertiva-zip-store-gate.sh"
rename_script "verify-lertiva-zip-corporate-prereqs.sh" "verify-lertiva-zip-corporate-prereqs.sh"
rename_script "verify-lertiva-zip-kobi-prereqs.sh" "verify-lertiva-zip-kobi-prereqs.sh"
rename_script "sync-vitrin-lertiva-zip-beta.sh" "sync-vitrin-lertiva-zip-beta.sh"
rename_script "agent-lertiva-zip-explorer-i1-i2-signoff.sh" "agent-lertiva-zip-explorer-i1-i2-signoff.sh"
rename_script "agent-lertiva-zip-kobi-signoff.sh" "agent-lertiva-zip-kobi-signoff.sh"
rename_script "agent-lertiva-zip-corporate-signoff.sh" "agent-lertiva-zip-corporate-signoff.sh"

W="$ROOT/scripts/windows"
for s in lertiva-zip-corporate-pilot-preflight.ps1 lertiva-zip-explorer-qa-preflight.ps1 lertiva-zip-kobi-pilot-preflight.ps1 pilot-lertiva-zip-kobi-start.ps1; do
  [ -f "$W/$s" ] || continue
  mv_if "$W/$s" "$W/${s/lertiva-zip-/lertiva-zip-}"
done

# --- Test source (opsiyonel okunabilirlik) ---
mv_if "$ROOT/crates/lertiva-archive/tests/lertiva_zip_interop.rs" "$ROOT/crates/lertiva-archive/tests/lertiva_zip_interop.rs"

echo "OK: tur2 git mv tamam"
