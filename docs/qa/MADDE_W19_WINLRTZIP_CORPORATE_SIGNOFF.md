# Madde W19 — winlrtZIP kurumsal pilot signoff %100

[W18](./MADDE_W18_WINLRTZIP_CORPORATE.md) GPO/denetim sonrası **saha pilot imza** kapısı (repo + isteğe Windows).

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Checklist | [WINLRTZIP_CORPORATE_PILOT_KOPYA.md](./WINLRTZIP_CORPORATE_PILOT_KOPYA.md) |
| Signoff | [WINLRTZIP_CORPORATE_PILOT_SIGNOFF.md](./WINLRTZIP_CORPORATE_PILOT_SIGNOFF.md) |
| Agent | `scripts/agent-winlrtzip-corporate-signoff.sh` |
| UI politika | Tauri `enterprise_policy_info` |
| Proxy | `cargo test audit::` |

## W19 %100 (repo kapısı)

1. `bash scripts/verify-madde-w19.sh`
2. Saha: preflight + checklist → agent signoff (isteğe)

Sonraki: Faz 5 split ZIP (W20 plan).
