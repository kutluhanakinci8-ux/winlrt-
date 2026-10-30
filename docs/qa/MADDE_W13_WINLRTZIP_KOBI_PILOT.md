# Madde W13 — Lertiva Zip Faz 2 KOBİ pilot %100

[WINLRTZIP_PLAN.md](../WINLRTZIP_PLAN.md) **Faz 2 kapısı:** Windows pilot — **KOBİ** en az 1 senaryo (e-posta eki).

## Senaryo (KOBİ)

| Adım | Beklenen |
|------|----------|
| Evrak klasörü sıkıştır | **E-posta profili** (≤10 MB hedef) — [MADDE_W7](./MADDE_W7_WINLRTZIP_EMAIL.md) |
| Alıcı açar | `test` / çıkarma hatasız |
| Explorer (isteğe) | **hızlı ZIP** veya UI e-posta profili |

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Proxy testler | `kobi_*` — `winlrtzip_interop.rs` |
| Preflight | `scripts/windows/winlrtzip-kobi-pilot-preflight.ps1` |
| Oturum | `scripts/windows/pilot-winlrtzip-kobi-start.ps1` |
| Checklist | [WINLRTZIP_KOBI_PILOT_KOPYA.md](./WINLRTZIP_KOBI_PILOT_KOPYA.md) |
| Signoff | `scripts/agent-winlrtzip-kobi-signoff.sh` |

## W13 %100 (repo kapısı)

1. `bash scripts/verify-madde-w13.sh`
2. Main pipeline: `kobi_`
3. Saha: Windows preflight + checklist → signoff (isteğe; vitrin pilot özeti ayrı)
