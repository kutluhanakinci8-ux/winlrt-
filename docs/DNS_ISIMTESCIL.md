# isimtescil — winlrt.com.tr DNS (Turhost VPS)

VPS: **37.230.105.99** · SSH port **23422** · hostname **srv.winlrt.com.tr**

Panel: [isimtescil.net](https://www.isimtescil.net) → **Domainlerim** → **winlrt.com.tr** → **DNS yönetimi**

---

## 1) Vitrin (ana site) — mevcut A kaydını düzenle

Tabloda **`winlrt.com.tr`** satırı, tip **A**, şu an muhtemelen `93.89.226.17`.

1. Satırın sağındaki **kalem (düzenle)**
2. **IP adresi** alanına **sadece** şunu yazın:

```text
37.230.105.99
```

3. Kaydet / onayla

Sonuç: **https://winlrt.com.tr** → VPS

**Dokunmayın:** `mail.winlrt.com.tr`, `ftp.winlrt.com.tr`, MX, NS, `www` CNAME (www köke bağlı kalır).

---

## 2) Sunucu adı (srv) — yoksa yeni A kaydı

Listede **`srv.winlrt.com.tr`** **A** kaydı yoksa:

**Gelişmiş DNS Ayarı Ekleyiniz**

| Alan | Yazılacak |
|------|-----------|
| İşlem | A Kaydı Oluştur |
| Kayıt ismi | `srv` |
| IP adresi | `37.230.105.99` |

**Yönlendir**

Zaten varsa ve IP farklıysa **düzenle** → `37.230.105.99`

---

## 3) Yazmayın

| Kayıt ismi | Oluşan adres | Neden |
|-------------|--------------|--------|
| `winlrt` | winlrt.winlrt.com.tr | Vitrin değil |
| `@` yeni kayıt (kök zaten var) | çakışma | Kökü **düzenleyin**, ikinci A eklemeyin |

---

## 4) Kontrol (10–30 dk sonra)

```bash
nslookup winlrt.com.tr
nslookup srv.winlrt.com.tr
```

İkisi de **37.230.105.99** göstermeli.

SSL sonrası nginx `server_name` hem kök hem **www** içermeli; yoksa certbot `www` için hata verir.

SSH (sonraki adım, DNS değil):

```bash
ssh -p 23422 root@37.230.105.99
```

veya

```bash
ssh -p 23422 root@srv.winlrt.com.tr
```
