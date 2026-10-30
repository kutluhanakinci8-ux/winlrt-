# winlrtRAR — GPO / kayıt defteri (Faz 4 beta)

Kurumsal dağıtım: **Group Policy Preferences** veya `.reg` ile aşağıdaki anahtarları hedefleyin.

## Kayıt yolu

```
HKLM\Software\winlrt\winlrtRAR
HKCU\Software\winlrt\winlrtRAR   (HKCU öncelikli okuma)
```

| Değer | Tür | Örnek | Etki |
|--------|-----|--------|------|
| `PasswordPolicy` | REG_SZ | `corporate` | Parola politikası (`consumer` / `corporate`) |
| `AuditMirrorPath` | REG_SZ | `\\fileserver\logs\winlrt\audit.jsonl` | Her denetim satırının kopyası |
| `ProductVersion` | REG_SZ | `0.1.0` | Envanter (kurulum yazar) |

Ortam değişkenleri (`WINLRT_PASSWORD_POLICY`, `WINLRT_AUDIT_MIRROR`) kayıt defterinden **önce** uygulanır.

## Örnek .reg

Repoda: `scripts/winlrt-corporate-policy-sample.reg`

## NSIS

Kurulum `ProductVersion` yazar; politika **zorlanmaz** (banka GPO ile set eder).

## Uygulama içi

Tauri: `enterprise_policy_info` → kaynak + aktif politika + audit mirror yolu.

İlgili: [WINLRTRAR_CORPORATE_POLICY.md](./WINLRTRAR_CORPORATE_POLICY.md) · [WINLRTRAR_AUDIT.md](./WINLRTRAR_AUDIT.md)
