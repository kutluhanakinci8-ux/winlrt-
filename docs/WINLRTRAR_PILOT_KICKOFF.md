# Lertiva Arc — kurumsal pilot kickoff (Faz 3)

Hedef segmentler (Faz 0): **banka**, **sendika**, **KOBİ**. Her pilot için aynı omurga, farklı vurgu.

## Ortak paket

1. **Kurulum:** GitHub Release beta veya imzalı build (Faz 3 kapısı)
2. **Politika:** `scripts/lertiva-arc-corporate-policy-sample.reg` veya ADMX `deploy/gpo/`
3. **Denetim:** `AuditMirrorPath` / isteğe `AuditForwardUrl`
4. **Modern crypto:** `RequireModernCrypto=1` + `PasswordPolicy=corporate`
5. **QA:** [WINLRTRAR_EXPLORER_QA.md](./WINLRTRAR_EXPLORER_QA.md)
6. **KPI kanıtı:** Windows benchmark artifact + vitrin `/benchmark/`

## Banka

| Madde | Not |
|-------|-----|
| GPO/ADMX | HKLM, merkezi audit mirror (UNC) |
| Parola | Kurumsal profil zorunlu (`--secure`) |
| SmartScreen | `TAURI_SIGNING_*` secret — imzalı MSI/NSIS |
| KVKK | `audit.jsonl` — parola yok; `path_hint` kısa |

## Sendika

| Madde | Not |
|-------|-----|
| Dağıtım | Toplu `.reg` veya GPO Preferences |
| Kullanım | Explorer çoklu seçim + hızlı `.wlrt` |
| Eğitim | Geçiş sihirbazı sekmesi (WinRAR birlikte okuma) |

## KOBİ

| Madde | Not |
|-------|-----|
| Kurulum | Tek makine NSIS; secret’sız beta + SmartScreen bilgisi |
| Benchmark | `/benchmark/` vitrin özeti |
| Destek | [WINLRTRAR_DURUM.md](./WINLRTRAR_DURUM.md) + GitHub Releases |

## Pilot kapanış (anket)

- [ ] Günlük iş akışı ≥ **8/10** (Faz 2 kapısı ile uyumlu)
- [ ] Windows resmi KPI: Balanced + Fast artifact incelendi
- [ ] En az bir şifreli `.wlrt` + bir Explorer «buraya aç» senaryosu

Operasyon: [WINLRTRAR_POST_MERGE.md](./WINLRTRAR_POST_MERGE.md) · [WINLRTRAR_PRODUCTION_CHECKLIST.md](./WINLRTRAR_PRODUCTION_CHECKLIST.md)
