# winlrt masaüstü

Windows ve macOS istemcisi burada geliştirilecek.

Önerilen stack (Turhost Ubuntu + GitHub):

- **Tauri 2** + TypeScript/React (küçük kurulum paketi)
- CI: GitHub Actions → `.exe` / `.dmg` → `apps/web/public/downloads/` veya VPS `releases/`

Kurulum (geliştirici makinesi veya VPS):

```bash
npm create tauri-app@latest
# veya mevcut monorepo'ya tauri init
```
