# Madde W18 — Lertiva Zip kurumsal (GPO + denetim) %100

Faz 4 başlangıç: [WINLRTZIP_PLAN.md](../WINLRTZIP_PLAN.md) — Lertiva Arc ile hizalı politika.

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| GPO ADMX/ADML | `deploy/gpo/Lertiva Zip.*` |
| Kayıt defteri | `Software\winlrt\Lertiva Zip` (+ Lertiva Arc yedek okuma) |
| Motor | `enterprise::init_registry_product("zip")`, `audit::init_audit_product("zip")` |
| UI denetim | `create_zip` / `extract_zip` → `audit.jsonl` |
| Vitrin ZIP | `Lertiva Zip-corporate-gpo.zip` |
| Pilot | [WINLRTZIP_CORPORATE_PILOT_KOPYA.md](./WINLRTZIP_CORPORATE_PILOT_KOPYA.md) |

## W18 %100 (repo kapısı)

1. `bash scripts/verify-madde-w18.sh`
2. `cargo test audit::` + `cargo check -p lertiva-zip`
3. Saha: preflight + checklist (isteğe signoff)
