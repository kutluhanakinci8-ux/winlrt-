# Madde W5 — Lertiva Zip parola UI + Explorer `--secure` %100

## Kapsam

| Bileşen | Konum |
|---------|--------|
| UI aç/çıkar/test parolası | `apps/lertiva-zip/src/App.tsx` |
| AES oluşturma + Explorer sihirbazı | `evaluate_create_password`, `startup_secure_wizard` |
| Explorer menüleri | `installer-hooks.nsh` (`--secure`) |
| Politika | `password_policy::policy_for_secure_create()` (kurumsal varsayılan) |

## Menüler (W5)

| Menü | Komut |
|------|--------|
| Lertiva Zip — şifreli sıkıştır | `--compress-folder "%1" --secure` |
| Lertiva Zip — seçilenler şifreli | `--compress-selection %* --secure` |

`--secure`: uygulama açılır → parola sihirbazı → AES ZIP oluştur.

## W5 %100

1. UI parola alanları + Explorer `--secure` NSIS satırları repo’da
2. `bash scripts/verify-madde-w5.sh`
3. Sonraki beta NSIS kurulumunda menüler yazılır (W3 CI hattı)
