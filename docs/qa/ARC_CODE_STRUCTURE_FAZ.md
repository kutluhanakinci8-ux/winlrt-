# Lertiva Arc — kod yapısı fazları (F0–F10)

**Tamamlanma (repo):** `bash scripts/verify-lertiva-arc-code-structure-100.sh`  
**Vitrin JSON:** `lertiva-arc-code-structure-gate.json`

| Faz | Odak | Çıktı |
|-----|------|--------|
| F0 | Boşluk | RAR/7z `extract_one`, manifest asset adı, tauri marka |
| F1 | Motor paralel | `.wlrt` rayon create/extract eşiği (mevcut), `compare-local-7z-winrar.ps1` |
| F2 | Akışlı önizleme | `preview_entry_bytes` zip/wlrt/rar/7z + Arc UI |
| F3 | Kurtarma | `recovery.rs`, CLI `recover-partial` / `extract-one` |
| F4 | Dedup | `LERTIVA_WLRT_DEDUP=1` → `dedup_file_list` create |
| F5 | Arc UI | Entry tree, tüm formatlarda tek dosya |
| F6 | Geçiş | Varsayılan uygulamalar düğmesi (mevcut) |
| F7 | Explorer | `lertiva-arc-explorer-qa-preflight.ps1` |
| F8 | Kurumsal | Mevcut GPO/enterprise (saha signoff ayrı) |
| F9 | Marka | `scan-brand-lertiva-arc.sh` |
| F10 | Kapı | `verify-lertiva-arc-code-structure-100.sh` |

Dış bağımlılık (bu doküman dışı): EV imza, macOS Arc, Lerta prod bearer.
