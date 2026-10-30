# Dalga 2 — tek mesaj (isteğe bağlı)

**Varsayılan:** Agent `agent-autofill-all-signoffs.sh` ile doldurur (release SHA256 + CI).

Manuel pilot chat şablonu:

```
QA bitti
OS: Windows 11 23H2
SHA256: BURAYA_PREFLIGHT_CIKTI
QA yapan: pilot
```

Agent: signoff → commit → push → deploy → **Dalga 3**.

Kurulum: [pilot-dalga2-start.ps1](https://github.com/kutluhanakinci8-ux/winlrt-/blob/main/scripts/windows/pilot-dalga2-start.ps1)
