#!/usr/bin/env bash
# Homebrew gh eski macOS'ta (Monterey altı) derlenemez — GitHub'dan hazır binary.
# Kullanım: bash scripts/mac-install-gh-portable.sh
set -euo pipefail

GH_VERSION="2.40.1"
INSTALL_DIR="${HOME}/.local/bin"
ARCH="$(uname -m)"
case "${ARCH}" in
  arm64) ZIP="gh_${GH_VERSION}_macOS_arm64.zip" ;;
  x86_64) ZIP="gh_${GH_VERSION}_macOS_amd64.zip" ;;
  *)
    echo "Desteklenmeyen mimari: ${ARCH}"
    exit 1
    ;;
esac

URL="https://github.com/cli/cli/releases/download/v${GH_VERSION}/${ZIP}"
TMP="$(mktemp -d)"
trap 'rm -rf "${TMP}"' EXIT

echo "İndiriliyor: ${URL}"
curl -fsSL "${URL}" -o "${TMP}/gh.zip"
unzip -q "${TMP}/gh.zip" -d "${TMP}"
mkdir -p "${INSTALL_DIR}"
cp "${TMP}/gh_${GH_VERSION}_macOS_${ARCH}/bin/gh" "${INSTALL_DIR}/gh"
chmod +x "${INSTALL_DIR}/gh"

if ! echo ":${PATH}:" | grep -q ":${INSTALL_DIR}:"; then
  echo ""
  echo "PATH'e ekleyin (~/.zshrc):"
  echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
  echo ""
fi

"${INSTALL_DIR}/gh" --version
echo ""
echo "Sonra:"
echo "  ${INSTALL_DIR}/gh auth login"
echo "  ${INSTALL_DIR}/gh secret set VPS_SSH_KEY --repo kutluhanakinci8-ux/winlrt- < ~/.ssh/winlrt-gha-deploy"
