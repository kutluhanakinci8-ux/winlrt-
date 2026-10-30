# Madde W12 — winlrtZIP Explorer I1/I2 %100

## Senaryolar

| ID | Senaryo | Otomasyon (CI) | Windows saha |
|----|---------|----------------|--------------|
| I1 | Explorer ile oluşturulan ZIP → winlrt aç | Info-ZIP `zip` → `zip_io` / CLI `test` | Sağ tık **winlrtZIP ile aç** |
| I2 | winlrt ZIP → Explorer “Tümünü çıkar” | winlrt create → `unzip` / 7-Zip | Explorer **Tümünü çıkar** uyarısız |

Explorer motoru CI’da yok; **Info-ZIP** ve **7-Zip** Windows Explorer davranışına yakın proxy kabul edilir (PKZIP / DEFLATE).

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Interop testleri | `crates/winlrt-archive/tests/winlrtzip_interop.rs` (`i1_*`, `i2_*`) |
| NSIS menüler | [MADDE_W4](./MADDE_W4_WINLRTZIP_EXPLORER.md) |
| Preflight | `scripts/windows/winlrtzip-explorer-qa-preflight.ps1` |
| Saha checklist | [WINLRTZIP_EXPLORER_QA_KOPYA.md](./WINLRTZIP_EXPLORER_QA_KOPYA.md) |
| Signoff | `scripts/agent-winlrtzip-explorer-i1-i2-signoff.sh` → [WINLRTZIP_EXPLORER_QA_SIGNOFF.md](./WINLRTZIP_EXPLORER_QA_SIGNOFF.md) |

## W12 %100 (repo kapısı)

1. `bash scripts/verify-madde-w12.sh`
2. Main pipeline: `i1_` + `i2_` (zip/unzip paketleri)
3. İsteğe: Windows pilot signoff (I1/I2 gerçek Explorer) — vitrin pilot durumu ayrı güncellenir
