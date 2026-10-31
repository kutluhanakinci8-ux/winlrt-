# G5 — Kurumsal pilot runbook (Arc + Zip)

**Hedef:** AD/GPO veya `.reg` ile politika, denetim kanıtı, imzalı signoff. Sertifika/mağaza bu fazda yok.

## 0) Önkoşul

- Faz 4 Explorer QA: [EXPLORER_QA_SIGNOFF.md](./EXPLORER_QA_SIGNOFF.md) PASS
- İndirme: [winlrt.com.tr/downloads/](https://winlrt.com.tr/downloads/)

## 1) Paketler

| Ürün | Vitrin ZIP | Repo |
|------|------------|------|
| Lertiva Arc | `/downloads/lertiva-arc-corporate-gpo.zip` | `deploy/gpo/lertiva-arc.admx` |
| Lertiva Zip | `/downloads/lertiva-zip-corporate-gpo.zip` | `deploy/gpo/lertiva-zip.admx` |

Doğrulama: `bash scripts/verify-dalga3-prereqs.sh` · `bash scripts/verify-lertiva-zip-corporate-prereqs.sh`

## 2) Windows IT — hızlı yol (test OU)

```powershell
# Arc
powershell -ExecutionPolicy Bypass -File scripts/windows/corporate-pilot-preflight.ps1
powershell -ExecutionPolicy Bypass -File scripts/windows/apply-corporate-sample.ps1

# Zip
powershell -ExecutionPolicy Bypass -File scripts/windows/lertiva-zip-corporate-pilot-preflight.ps1
```

ADMX: `C:\Windows\PolicyDefinitions\` + Group Policy Editor → Lertiva şablonları.

## 3) Senaryolar (~30 dk)

1. Zayıf parola → red (şifreli oluşturma / `--secure`)
2. Güçlü parola → `.wlrt` veya ZIP AES
3. `audit.jsonl` satırı (parola alanı **yok**)
4. İsteğe: HTTP audit forward (kurumsal URL GPO)

Detay: [WINLRTZIP_CORPORATE_PILOT_KOPYA.md](./WINLRTZIP_CORPORATE_PILOT_KOPYA.md) · [DALGA3_KURUMSAL_KOPYA.md](./DALGA3_KURUMSAL_KOPYA.md)

## 4) Signoff

| Dosya | Kim doldurur |
|-------|----------------|
| [KURUMSAL_PILOT_SIGNOFF.md](./KURUMSAL_PILOT_SIGNOFF.md) | Arc pilot (repo PASS mevcut) |
| [WINLRTZIP_CORPORATE_PILOT_SIGNOFF.md](./WINLRTZIP_CORPORATE_PILOT_SIGNOFF.md) | **Saha IT** (G5 %100 için gerekli) |

Rehber: [G5_SAHA_SIGNOFF_REHBERI.md](./G5_SAHA_SIGNOFF_REHBERI.md)

## 5) Repo / vitrin kapısı

```bash
bash scripts/verify-faz-g5.sh
bash scripts/evaluate-lertiva-corporate-gate.sh
bash scripts/gap-phase-finish.sh 5
```

JSON: `lertiva-corporate-pilot-gate.json` · Pano: [/benchmark/corporate/](/benchmark/corporate/)
