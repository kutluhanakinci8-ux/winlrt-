#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"
HTML="$ROOT/apps/web/public/downloads/index.html"

TAG="$(gh release list --repo "$REPO" --limit 40 --json tagName,isPrerelease \
  | jq -r '[.[] | select(.isPrerelease and (.tagName | startswith("winlrt-zip-beta.")))] | .[0].tagName // empty')"
test -n "$TAG"
ASSET="$(gh release view "$TAG" --repo "$REPO" --json assets \
  | jq -r '.assets[] | select(.name | endswith("-setup.exe")) | .name' | head -1)"
test -n "$ASSET"

export HTML TAG ASSET REPO
python3 <<'PY'
import os, re
from pathlib import Path
html = Path(os.environ["HTML"]).read_text()
tag = os.environ["TAG"]
asset = os.environ["ASSET"]
repo = os.environ["REPO"]
url = f"https://github.com/{repo}/releases/download/{tag}/{asset}"
tag_url = f"https://github.com/{repo}/releases/tag/{tag}"
block = f"""      <ul class="product-list">
        <li>
          <div>
            <strong>Windows 10+ (64-bit) — beta</strong>
            <div class="meta">
              <a href="{url}">İndir: {asset}</a>
              · Imzasız beta ·
              <a href="{tag_url}">{tag}</a>
            </div>
          </div>
          <span class="status status-dev">Beta</span>
        </li>
        <li>
          <div>
            <strong>macOS 12+ (Apple Silicon &amp; Intel)</strong>
            <div class="meta">Disk imajı / paket</div>
          </div>
          <span class="status status-dev">Yakında</span>
        </li>
      </ul>"""
pat = (
    r'(<h2 class="section-label" style="margin-bottom: 1rem">winlrtZIP</h2>\s*'
    r'<p class="meta" style="margin-bottom: 1rem">.*?</p>\s*)'
    r'<ul class="product-list">.*?</ul>'
)
new, n = re.subn(pat, r"\1" + block, html, count=1, flags=re.S)
if n != 1:
    raise SystemExit("vitrin winlrtZIP bloğu güncellenemedi")
Path(os.environ["HTML"]).write_text(new)
PY
echo "OK: vitrin winlrtZIP → $TAG"
