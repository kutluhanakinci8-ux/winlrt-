# `.wlrt` container format v0

Faz 1 minimum spesifikasyon — winlrtRAR / winlrtZIP ortak yerli yazma formatı.

## Dosya düzeni

```text
[header 10 byte]
[entry] × file_count
[footer 4 byte]
```

### Header

| Alan | Boyut | Değer |
|------|-------|--------|
| magic | 4 | `WLRT` |
| version | u16 LE | `0` |
| file_count | u32 LE | girdi sayısı |

### Entry (tekrar)

| Alan | Boyut | Açıklama |
|------|-------|----------|
| path_len | u16 LE | UTF-8 yol uzunluğu |
| path | path_len | `/` ayraç, göreli yol |
| raw_size | u64 LE | sıkıştırılmamış boyut |
| comp_size | u64 LE | gövde boyutu |
| method | u8 | `1` = zstd |
| level | u8 | zstd seviyesi (profil) |
| payload | comp_size | zstd sıkıştırılmış veri |

### Footer

| magic | 4 | `TRLW` |

## Profiller

`CompressionProfile` → zstd seviyesi ([profile.rs](../crates/winlrt-archive/src/profile.rs)).

## Sürümleme

v1+: merkezi dizin, şifreleme, imza — Faz 2+.
