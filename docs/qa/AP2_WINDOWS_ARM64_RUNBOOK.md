# AP-2 — Lertiva Arc Windows ARM64

**Hedef:** `Lertiva Arc_{ver}_arm64-setup.exe` (NSIS), ARM64 PE (`Machine=0xAA64`), GitHub Release asset ve vitrin manifest `windows_arm64`.

## CI

- Workflow: `.github/workflows/lertiva-arc-windows-arm64.yml`
- Tetik: `.github/flags/trigger-arc-arm64-build` push (main), tag `v1.0.*` veya Actions → Run workflow
- Build: `windows-latest` üzerinde çapraz derleme (`aarch64-pc-windows-msvc`), önce `unrar-embedded`, olmazsa gömülü RAR olmadan yeniden dener (uyarı verir)
- İmza: AP-1 secret varsa `sign-windows-nsis-bundle.ps1` ARM64 setup.exe'yi imzalar
- Doğrulama: uygulama exe PE başlığı ARM64 olmalı

## Kapı

```bash
bash scripts/evaluate-lertiva-arc-platform-gate.sh
jq '.phases["AP-2"]' apps/web/public/benchmark/samples/lertiva-arc-platform-gate.json
```

`AP-2 pass` = workflow'un `main` üzerinde en az bir başarılı koşusu.

## İnsan (kısa)

- ARM64 Windows cihaz veya VM'de kurulum smoke (Explorer sağ tık opsiyonel)
- Imzalı yayın AP-1 secret'ına bağlı

Sonraki: [ARC_PLATFORM_FAZ_PLAN.md](./ARC_PLATFORM_FAZ_PLAN.md) AP-3 (macOS).
