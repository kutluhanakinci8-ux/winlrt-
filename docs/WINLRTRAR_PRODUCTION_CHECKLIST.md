# winlrtRAR — production checklist (Faz 3 → v1.0)

Sırayla tamamlanacak maddeler; Cloud Agent ve sizin manuel adımlar birlikte.

## 1. Kod ve dal birleştirme

- [ ] Feature dalları `main`’e merge — sıra: [WINLRTRAR_BRANCH_MERGE.md](./WINLRTRAR_BRANCH_MERGE.md)
- [ ] `main` push sonrası **Main pipeline** yeşil (vitrin build + rust test)

## 2. GitHub Secrets

| Secret | Amaç |
|--------|------|
| `VPS_HOST`, `VPS_PORT`, `VPS_USER` | winlrt.com.tr deploy |
| `VPS_SSH_KEY` veya `VPS_SSH_KEY_B64` veya `VPS_SSH_PASSWORD` | SSH (repoda **deploy anahtarı yok**) |
| `VPS_WEB_ROOT` | Örn. `/var/www/winlrt/apps/web/dist` |
| `TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | İmzalı Windows beta / v1.0 |

**Repository variable (isteğe bağlı):** `WINLRT_KPI_STRICT=1` → main/Linux ve Windows benchmark job KPI kapısında fail.

Detay: [AUTO_DEPLOY.md](./AUTO_DEPLOY.md) · Mac: `scripts/mac-set-github-vps-secret.sh`

**Güvenlik:** Eski repodaki deploy private key commit edildiyse VPS’te `authorized_keys`’ten kaldırın ve yeni anahtar kullanın.

## 3. Windows benchmark (Faz 3 kapısı)

- [ ] Actions → **winlrt Windows benchmark (resmi)** → Run workflow
- [ ] Kapı doğrulama: aynı ekranda **KPI kapısı geçmezse job başarısız** = açık
- [ ] Artifact `bench-report-windows`: `compression_gate` (Balanced) ve `speed_gate` (Fast JSON)
- [ ] CI log: `scripts/evaluate-bench-kpi.ps1` özeti
- [ ] Vitrin: [/benchmark/](https://winlrt.com.tr/benchmark/) canlı KPI kartları (samples JSON)
- [ ] Artifact’tan vitrin: `bash scripts/import-windows-kpi-to-vitrin.sh bench-report-windows.json bench-report-windows-fast.json`

## 4. winlrtRAR Windows beta

- [ ] Actions → **winlrtRAR Windows beta build** → Run workflow
- [ ] Secret yoksa CI uyarısı beklenir; secret varsa: `verify-windows-signature.ps1` geçer
- [ ] Test build: *Yalnızca artifact* ile Release atlamak mümkün (`skip_release`)
- [ ] GitHub Release artifact indir → pilot makinede kurulum + Explorer menü testi

## 5. Explorer pilot QA

- [ ] [WINLRTRAR_EXPLORER_QA.md](./WINLRTRAR_EXPLORER_QA.md) maddeleri (NSIS beta kurulum)

## 6. Kurumsal pilot

- [ ] ADMX: `deploy/gpo/winlrtRAR.admx` → PolicyDefinitions
- [ ] Örnek politika: `scripts/winlrt-corporate-policy-sample.reg`
- [ ] Denetim mirror / HTTP forward test (isteğe bağlı)

## 7. v1.0 kapısı

- [ ] KPI Windows’ta geçti (benchmark artifact)
- [ ] İmzalı kurulum yayında
- [ ] En az 1 kurumsal pilot onayı

İlgili: [WINLRTRAR_CODE_SIGNING.md](./WINLRTRAR_CODE_SIGNING.md) · [WINLRTRAR_PLAN.md](./WINLRTRAR_PLAN.md)
