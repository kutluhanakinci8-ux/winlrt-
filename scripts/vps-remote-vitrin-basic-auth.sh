#!/usr/bin/env bash
# VPS üzerinde çalışır (stdin ile değil, doğrudan kopyalanır).
set -euo pipefail
MODE="${1:?mode enable|disable}"
AUTH_USER="${2:?user}"
AUTH_PASS="${3:-}"

SNIP="/etc/nginx/snippets/winlrt-basic-auth.conf"
HTPASS="/etc/nginx/.htpasswd-vitrin"
MARK="snippets/winlrt-basic-auth.conf"
CONF="/etc/nginx/sites-enabled/winlrt.conf"

install -d -m 755 /etc/nginx/snippets

ensure_video_public() {
  if [ ! -f "$CONF" ]; then
    return 0
  fi
  if grep -q 'location \^~ /assets/video/' "$CONF"; then
    return 0
  fi
  python3 - "$CONF" <<'PY'
import sys
path = sys.argv[1]
mark = "    location ^~ /assets/video/"
block = """
    # Demo videolar — basic auth altında <video> oynatıcı için
    location ^~ /assets/video/ {
        auth_basic off;
        expires 7d;
        add_header Cache-Control "public";
        try_files $uri =404;
    }

"""
text = open(path, encoding="utf-8").read()
if mark.strip() in text:
    sys.exit(0)
needle = "    location / {"
if needle not in text:
    print("nginx: location / bulunamadı, video bloğu eklenemedi", file=sys.stderr)
    sys.exit(1)
open(path, "w", encoding="utf-8").write(text.replace(needle, block + needle, 1))
print("nginx: /assets/video/ public location eklendi")
PY
}

ensure_include() {
  if [ ! -f "$CONF" ]; then
    echo "nginx site yok: $CONF" >&2
    exit 1
  fi
  if grep -q "$MARK" "$CONF"; then
    return 0
  fi
  python3 - "$CONF" "$MARK" <<'PY'
import re, sys
path, mark = sys.argv[1], sys.argv[2]
text = open(path, encoding="utf-8").read()
line = f"    include /etc/nginx/{mark};\n"
if mark in text:
    sys.exit(0)

def patch_server(block: str) -> str:
    if mark in block:
        return block
    if re.search(r"^\s*access_log", block, re.M):
        block, n = re.subn(
            r"(^\s*access_log[^\n]*\n)",
            r"\1" + line,
            block,
            count=1,
            flags=re.M,
        )
        if n:
            return block
    return block.replace("{\n", "{\n" + line, 1)

out = []
i = 0
while i < len(text):
    m = re.match(r"server\s*\{", text[i:])
    if not m:
        out.append(text[i])
        i += 1
        continue
    start = i + m.start()
    depth = 0
    j = i + m.start()
    while j < len(text):
        if text[j] == "{":
            depth += 1
        elif text[j] == "}":
            depth -= 1
            if depth == 0:
                j += 1
                break
        j += 1
    block = text[start:j]
    out.append(patch_server(block))
    i = j
open(path, "w", encoding="utf-8").write("".join(out))
PY
}

if [ "$MODE" = "disable" ]; then
  printf '%s\n' "# Vitrin basic auth kapalı" >"$SNIP"
  rm -f "$HTPASS"
  ensure_include
  ensure_video_public
  nginx -t
  systemctl reload nginx
  echo "OK: vitrin basic auth kapalı"
  exit 0
fi

if [ -z "$AUTH_PASS" ]; then
  echo "Şifre boş" >&2
  exit 1
fi

command -v htpasswd >/dev/null 2>&1 || apt-get install -y -qq apache2-utils
htpasswd -bc "$HTPASS" "$AUTH_USER" "$AUTH_PASS"
chmod 640 "$HTPASS"
chown root:www-data "$HTPASS" 2>/dev/null || chown root:nginx "$HTPASS" 2>/dev/null || true
cat >"$SNIP" <<EOF
auth_basic "Lertiva — erken erişim";
auth_basic_user_file ${HTPASS};
EOF
ensure_include
ensure_video_public
nginx -t
systemctl reload nginx
echo "OK: vitrin basic auth açık (kullanıcı: ${AUTH_USER})"
