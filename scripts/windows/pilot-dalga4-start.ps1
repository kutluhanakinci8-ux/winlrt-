# Dalga 4 — v1.0 kapı checklist (iki signoff sonrası).
$ErrorActionPreference = 'Continue'

Write-Host '=== Lertiva Arc Dalga 4 (v1.0 kapı) ==='
Write-Host 'Onkosul: EXPLORER_QA_SIGNOFF + KURUMSAL_PILOT_SIGNOFF = PASS (repo docs/qa/).'
Write-Host ''
Write-Host 'Kapilar:'
Write-Host '  [ ] Windows KPI vitrin + resmi bench artifact'
Write-Host '  [ ] Explorer QA signoff'
Write-Host '  [ ] Kurumsal pilot signoff'
Write-Host '  [ ] TAURI imzali kurulum (veya imzasiz istisna dokumante)'
Write-Host ''
Write-Host 'Rehber: docs/qa/DALGA4_V1_OTURUM.md'
Write-Host 'Agent (Linux): bash scripts/check-v1-gate-status.sh'
Write-Host ''
Write-Host 'Chat (signofflar PASS iken):'
Write-Host '  v1.0 hazir'
Write-Host '  Tag: v1.0.0'
Write-Host '  Not: <release notes ozeti>'

$doc = 'https://github.com/kutluhanakinci8-ux/winlrt-/blob/main/docs/qa/DALGA4_V1_OTURUM.md'
Start-Process $doc
