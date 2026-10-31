# G5 — saha signoff doldurma rehberi (IT)

Agent **Zip saha** satırını sizin adınıza PASS yapamaz; tabloyu doldurup commit veya PR gönderin.

## WINLRTZIP_CORPORATE_PILOT_SIGNOFF.md

| Alan | Örnek |
|------|--------|
| Segment | banka / sendika / KOBİ |
| Tarih | 2026-11-… |
| ADMX / .reg | «GPO import OU=Pilot» veya «sample.reg HKLM test» |
| Denetim testi | «audit.jsonl create+extract satırı, parola yok» |
| Onaylayan | Ad soyad, unvan |

Son satır: **`[x] PASS`** · `[ ] FAIL` boş bırakın.

## Kanıt (isteğe bağlı)

- Preflight çıktısı (PowerShell)
- GPO raporu ekran görüntüsü
- Audit satırı (parola maskeli)

## Commit sonrası

```bash
bash scripts/evaluate-lertiva-corporate-gate.sh
bash scripts/gap-phase-finish.sh 5
```

`lertiva-corporate-pilot-gate.json` içinde `zip_field_signoff` ve `all_pass` yeşile döner.
