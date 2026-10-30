# winlrt — `main` merge sonrası sıra (güncel)

Durum: **A→L + Post-v1 tamam** (imzasız v1.0.0). Sertifika **ertelendi**.  
Sıradaki (sertifikasız): [POST_V2_SIRA.md](./qa/POST_V2_SIRA.md)

## 1. GitHub Actions (otomatik)

- [x] **Main pipeline** — `main` push (web + rust)
- [x] **Windows benchmark** — flag veya push; Balanced strict
- [x] **Lertiva Arc beta** — flag build + release ([beta.44+](https://github.com/kutluhanakinci8-ux/winlrt-/releases))

## 2. Secrets (isteğe bağlı)

| Secret | Durum |
|--------|--------|
| `VPS_*` | Agent + VPS cron **canlı**; panel secret → CI deploy ekstra |
| `TAURI_SIGNING_*` | **Ertelendi** — imzasız resmi yeterli |

Rehber: [DALGA1_GITHUB_VPS_SECRET.md](./DALGA1_GITHUB_VPS_SECRET.md) · imza: [WINLRTRAR_CODE_SIGNING.md](./WINLRTRAR_CODE_SIGNING.md)

## 3. Vitrin / KPI

- [x] Windows resmi KPI vitrin JSON
- [ ] Fast hız kapısı tam (Madde Q — `text` fixture; bloklayıcı değil)

```bash
bash scripts/agent-madde-n-complete.sh
```

## 4. Doğrulama (tek komut)

```bash
bash scripts/verify-all-maddes.sh
```

## 5. Yerel smoke

```bash
bash scripts/post-merge-smoke.sh
```
