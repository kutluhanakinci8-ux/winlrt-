# Lertiva Arc — denetim günlüğü (Faz 4 beta)

JSON Lines dosyası; **parola / anahtar metni asla yazılmaz**.

## Konum

| Ortam | Varsayılan yol |
|--------|----------------|
| Windows | `%APPDATA%\Lertiva Arc\audit.jsonl` |
| Linux/macOS | `~/.local/share/Lertiva Arc/audit.jsonl` |

Override: `WINLRT_AUDIT_LOG=/yol/audit.jsonl`  
Kapatma: `WINLRT_AUDIT_DISABLE=1`  
**Merkezi kopya:** `WINLRT_AUDIT_MIRROR` veya GPO `AuditMirrorPath`  
**HTTP iletim (beta):** `WINLRT_AUDIT_FORWARD_URL` veya GPO `AuditForwardUrl` — satır başına POST (`Content-Type: application/x-ndjson`); hata uygulamayı durdurmaz.

(bkz. [WINLRTRAR_GPO.md](./WINLRTRAR_GPO.md))

Tauri: `audit_log_path` komutu.

## Olaylar

| event | Açıklama |
|--------|-----------|
| `archive_create` | Oluşturma (yalnız dosya adı ipucu) |
| `archive_extract` | Açma |
| `secure_password_prompt` | Win32 / iptal / kabul |
| `password_policy_reject` | Politika reddi |

## Örnek satır

```json
{"ts_unix":1730000000,"event":"archive_create","channel":"explorer_native","ok":true,"path_hint":"Projem.wlrt","format":"wlrt","policy_id":"corporate"}
```

KVKK: günlükte tam yol isteğe bağlı kısaltılır (`path_hint` = dosya adı).
