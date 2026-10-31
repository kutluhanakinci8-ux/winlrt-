# G8 — Lerta denetim kayıtları (audit.jsonl)

Motor: `crates/lertiva-archive/src/audit.rs` · Olaylar **metadata-only**; ZIP/wlrt içeriği loglanmaz.

## Olay türleri

| `event` | Ne zaman |
|---------|----------|
| `lerta_mail_attachment` | W28 posta eki hazırlama / doğrulama |
| `lerta_message_attachment` | W30 mesaj eki hazırlama / doğrulama |
| `lerta_api` | W29/W31 HTTP metadata gönderimi veya dry-run |

## Örnek satır (maskeli)

```json
{"ts":"…","event":"lerta_api","channel":"zip","ok":true,"detail":"dry_run_submit"}
```

## IT kontrol listesi

- [ ] `detail` içinde parola / bearer yok
- [ ] Canlı gönderimde yalnızca hash + boyut + sözleşme sürümü
- [ ] KVKK: içerik cihazda (`metadata_only: true` vitrin JSON)

Zip Tauri: `audit_log_path` komutu ile dosya konumu.
