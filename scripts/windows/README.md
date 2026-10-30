# Windows pilot scriptleri

| Script | Dalga | Açıklama |
|--------|-------|----------|
| `pilot-dalga2-start.ps1` | 2 | Beta release + preflight yönlendirme |
| `explorer-qa-preflight.ps1` | 2 | Kurulum / Explorer kayıt defteri + SHA256 |
| `pilot-dalga3-start.ps1` | 3 | GPO ZIP + örnek `.reg` (Explorer PASS sonrası) |
| `apply-corporate-sample.ps1` | 3 | HKLM örnek politika |
| `corporate-pilot-preflight.ps1` | 3 | Politika okuma |
| `pilot-dalga4-start.ps1` | 4 | v1.0 kapı checklist (signoff sonrası) |

Çalıştırma: `powershell -ExecutionPolicy Bypass -File <script>`

Chat şablonları: [docs/qa/PILOT_CHAT_SABLONLARI.md](../../docs/qa/PILOT_CHAT_SABLONLARI.md)
