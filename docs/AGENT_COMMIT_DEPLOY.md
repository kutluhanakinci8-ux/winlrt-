# Cloud Agent — commit · merge · deploy (her tur)

**Anlaşma:** Pilot / kullanıcı içerik verir; **agent** her turda:

1. `main` üzerinde değişiklikleri **commit** eder  
2. **`git push origin main`** (merge zaten `main`’de; feature gerekiyorsa `main`’e merge edip push)  
3. **`bash scripts/post-phase-deploy.sh`** (ortam `VPS_*` — canlı vitrin)

Kullanıcıdan **ayrı commit/push/deploy istenmez.**

## Dalga 2 (Explorer QA)

- Pilot Windows’ta test eder; sonucu chat’e yazar veya signoff alanlarını doldurur.  
- **Agent:** `docs/qa/EXPLORER_QA_SIGNOFF.md` günceller → commit → push → deploy.

## Dalga 3+

Aynı döngü: signoff / vitrin / docs → agent commit → push → deploy.
