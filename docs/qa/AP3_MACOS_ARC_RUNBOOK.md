# AP-3 — Lertiva Arc macOS (.dmg)

**Hedef:** `Lertiva Arc_{ver}_macos-universal.dmg` (Apple Silicon + Intel), Finder ile ZIP / .wlrt / RAR / 7z açma, vitrin manifest `macos`.

## CI

- Workflow: `.github/workflows/lertiva-arc-macos.yml` (`macos-14`)
- Tetik: `.github/flags/trigger-arc-macos-build` push, tag `v1.0.*`, Actions → Run workflow
- Build: `--target universal-apple-darwin --bundles dmg`; önce `unrar-embedded`, olmazsa gömülü RAR olmadan yeniden dener
- Doğrulama: `lipo -archs` çıktısında `arm64` ve `x86_64`
- Notarize: `APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`, `APPLE_SIGNING_IDENTITY`, `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID` varsa Tauri imzalar ve notarize eder (Zip ile aynı secret'lar)

## Kapsam kararı

macOS'ta Explorer benzeri sağ tık menüsü yok. Ürün metni: Finder dosya ilişkileri + sürükle-bırak. Win32 parola penceresi yerine uygulama içi parola alanı kullanılır.

## Kapı

```bash
bash scripts/evaluate-lertiva-arc-platform-gate.sh
jq '.phases["AP-3"]' apps/web/public/benchmark/samples/lertiva-arc-platform-gate.json
```

`AP-3 pass` = workflow'un `main` üzerinde en az bir başarılı koşusu. `notarized` = `lertiva-apple-notarize-configured.flag` (Apple Developer hesabı sonrası, G3 runbook).

## İnsan

- Apple Developer hesabı + `APPLE_*` secret'ları → [G2_G3_SIRALI_RUNBOOK.md](./G2_G3_SIRALI_RUNBOOK.md) Bölüm B
- Mac'te dmg açılış / Gatekeeper testi

Sonraki: AP-4 (Linux).
