# Faz 4 — Explorer pilot QA (sıra)

## 1. Beta indir

1. **Güncel beta:** https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/winlrt-rar-beta.41064653814  
   (`Lertiva Arc_0.1.0_x64-setup.exe`)
2. Veya Actions → **Lertiva Arc Windows beta build** → artifact **winlrt-rar-windows-bundle**
3. Kur: `Lertiva Arc_*_x64-setup.exe` (SmartScreen uyarısı — imza secret yoksa normal)
4. Preflight: `scripts/windows/explorer-qa-preflight.ps1` (kayıt defteri + exe SHA256)

## 2. Checklist

[WINLRTRAR_EXPLORER_QA.md](../WINLRTRAR_EXPLORER_QA.md) — tüm maddeleri sırayla işaretleyin.

## 3. Kayıt

Chat’e **QA bitti** + preflight SHA256 → agent [EXPLORER_QA_SIGNOFF.md](./EXPLORER_QA_SIGNOFF.md) commit · push · deploy.

## 4. Sorun

FAIL notları + ekran görüntüsü; agent veya yeni issue.
