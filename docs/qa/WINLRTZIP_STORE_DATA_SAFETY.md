# winlrtZIP — Google Play Data safety (şablon, W27)

Mağaza formu için özet; **bulut yedekleme yok**, veri cihazda / kullanıcı kanalında.

| Veri türü | Toplanır? | Amaç |
|-----------|-----------|------|
| Arşiv dosya adları (path_hint) | İsteğe bağlı, yerel audit | Kurumsal denetim |
| Parola / arşiv içeriği | Hayır | — |
| Konum / rehber | Hayır | — |
| Analitik | Hayır (varsayılan) | — |

**Şifreleme:** AES ZIP (kullanıcı parolası); paylaşım paketi KVKK yerel manifest (W25).

**Güncelleme:** GitHub Releases / kurumsal dağıtım (GPO); otomatik buluta yükleme yok.
