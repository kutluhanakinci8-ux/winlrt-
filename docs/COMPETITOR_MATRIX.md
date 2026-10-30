# Rekabet özellik matrisi (Faz 0–1)

WinRAR / WinZip / 7-Zip / Bandizip karşılaştırması — winlrtRAR ve winlrtZIP hedefleri.

**Lejant:** ✅ var · 🟡 planlı · ❌ yok / bilinçli yok · — inceleniyor

| Özellik | WinRAR | WinZip | 7-Zip | Bandizip | winlrtRAR hedef | winlrtZIP hedef |
|---------|--------|--------|-------|----------|-----------------|-----------------|
| ZIP okuma/yazma | ✅ | ✅ | ✅ | ✅ | ✅ Faz 1 | ✅ Faz 1 |
| ZIP64 | ✅ | ✅ | ✅ | ✅ | 🟡 Faz 1 hazırlık | 🟡 Faz 1 |
| RAR okuma | ✅ | kısıtlı | ✅ | ✅ | 🟡 **Faz 2** | miras |
| RAR yazma | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| 7z okuma | ❌ | ❌ | ✅ | ✅ | 🟡 Faz 2–3 | ikincil |
| Yerli `.wlrt` | ❌ | ❌ | ❌ | ❌ | ✅ **Faz 1** | ✅ preset |
| Çok çekirdek | kısıtlı | kısıtlı | ✅ | ✅ | ✅ Faz 1 | ✅ |
| AES şifre | ✅ | ✅ | ✅ | ✅ | 🟡 Faz 2–3 | 🟡 Faz 2–3 |
| Kurumsal GPO/MSI | ✅ | ✅ | — | — | 🟡 Faz 5–6 | 🟡 Faz 4 |
| Deneme ekranı | ✅ | ✅ | ❌ | ❌ | ❌ bilinçli | ❌ bilinçli |
| Bulut paylaşım | ❌ | ✅ Share | ❌ | ✅ | 🟡 Faz 5 opsiyonel | 🟡 Faz 5 KVKK |
| CLI / otomasyon | ✅ | sınırlı | ✅ | ✅ | ✅ Faz 1 `--json` | ✅ |
| Lerta posta eki | ❌ | ❌ | ❌ | ❌ | 🟡 **Faz 8** | 🟡 **Faz 8** |

## KPI (P0 — her iki ürün)

| KPI | Ölçüm |
|-----|--------|
| Hız | `cargo run -p winlrt-archive -- bench --all` + yerel 7-Zip/WinRAR |
| Sıkıştırma | `.wlrt` vs ZIP boyut; profil `Maximum` |
| Kurumsal güven | AES + denetim checklist (Faz 2+) |

Güncelleme: Faz 1 sprint sonrası — otomatik benchmark CI artifact `bench-report`.
