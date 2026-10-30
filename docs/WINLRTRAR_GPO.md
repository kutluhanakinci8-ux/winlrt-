# Lertiva Arc — GPO / kayıt defteri (Faz 4 beta)

Kurumsal dağıtım: **Group Policy Preferences** veya `.reg` ile aşağıdaki anahtarları hedefleyin.

## Kayıt yolu

```
HKLM\Software\Lertiva\Lertiva Arc
HKCU\Software\Lertiva\Lertiva Arc   (HKCU öncelikli okuma)
```

| Değer | Tür | Örnek | Etki |
|--------|-----|--------|------|
| `PasswordPolicy` | REG_SZ | `corporate` | Parola politikası (`consumer` / `corporate`) |
| `AuditMirrorPath` | REG_SZ | `\\fileserver\logs\winlrt\audit.jsonl` | Her denetim satırının kopyası |
| `AuditForwardUrl` | REG_SZ | `https://siem.example/api/winlrt-audit` | Denetim satırını HTTP POST (beta) |
| `RequireModernCrypto` | REG_DWORD | `1` | Kurumsal şifreli arşiv: Argon2id + ZIP AES-256 |
| `ProductVersion` | REG_SZ | `0.1.0` | Envanter (kurulum yazar) |

Ortam değişkenleri (`WINLRT_PASSWORD_POLICY`, `WINLRT_AUDIT_MIRROR`, `WINLRT_AUDIT_FORWARD_URL`) kayıt defterinden **önce** uygulanır.

## ADMX (Group Policy Administrative Templates)

Repoda: `deploy/gpo/lertiva-arc.admx` + `deploy/gpo/en-US/lertiva-arc.adml` + `deploy/gpo/tr-TR/lertiva-arc.adml`

**Merkezi mağaza (önerilen):**

1. `%SystemRoot%\PolicyDefinitions\lertiva-arc.admx`
2. `%SystemRoot%\PolicyDefinitions\en-US\lertiva-arc.adml` (ve `tr-TR` isteğe bağlı)
3. `gpupdate /force`

GPO yolu: **Computer Configuration** veya **User Configuration** → **Policies** → **Administrative Templates** → **Windows Components** → **Lertiva Arc**

Politikalar: parola profili, denetim mirror yolu, HTTP forward URL.

## Örnek .reg

Repoda: `scripts/lertiva-arc-corporate-policy-sample.reg`

## NSIS

Kurulum `ProductVersion` yazar; politika **zorlanmaz** (banka GPO ile set eder).

## Uygulama içi

Tauri: `enterprise_policy_info` → kaynak + aktif politika + audit mirror yolu.

İlgili: [WINLRTRAR_CORPORATE_POLICY.md](./WINLRTRAR_CORPORATE_POLICY.md) · [WINLRTRAR_AUDIT.md](./WINLRTRAR_AUDIT.md)
