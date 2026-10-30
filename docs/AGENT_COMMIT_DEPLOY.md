# Cloud Agent — commit · merge · deploy (her tur)

**Anlaşma:** Pilot / kullanıcı içerik verir; **agent** her turda:

1. `main` üzerinde değişiklikleri **commit** eder  
2. **`git push origin main`** (merge zaten `main`’de; feature gerekiyorsa `main`’e merge edip push)  
3. **`bash scripts/post-phase-deploy.sh`** (ortam `VPS_*` — canlı vitrin)

Kullanıcıdan **ayrı commit/push/deploy istenmez.**

## Dalga 2 (Explorer QA)

- Pilot Windows’ta test eder; chat’e şablon yazar ([PILOT_CHAT_SABLONLARI.md](./qa/PILOT_CHAT_SABLONLARI.md)).  
- **Agent:** `scripts/agent-parse-pilot-chat.sh` (stdin) veya `agent-explorer-signoff.sh` → commit → push → deploy → Dalga 3.

## Dalga 3 (kurumsal)

Explorer PASS sonrası pilot ADMX/.reg test eder → chat **kurumsal pilot bitti** → agent `scripts/agent-corporate-signoff.sh` → commit · push · deploy.

## Dalga 4 (v1.0)

İki signoff PASS → chat **v1.0 hazir** → `agent-parse-pilot-chat.sh` → `V1_RELEASE_DRAFT.md` → commit · push · deploy · (onayla) tag.

## Dalga 5+

Aynı döngü: signoff / vitrin / docs → agent commit → push → deploy.
