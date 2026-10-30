# Madde W25 — winlrtZIP güvenli paylaşım (KVKK smoke) %100

Faz 5: WinZip Share alternatifi — **yerel AES paket + manifest**, bulut zorunlu değil.

## Kapsam (W25)

| Özellik | Durum |
|---------|--------|
| AES ZIP (e-posta profili) | `create_secure_share_pack` |
| KVKK manifest | `{stem}.winlrtshare.json`, `kvkk_local_only: true` |
| Self-hosted URL şablonu | `self_hosted_download_url` |
| İsteğe imza | `WINLRT_SIGNING_SEED_HEX` varsa `.winlrtsig.json` |
| Denetim | `secure_share_pack` audit |
| CLI | `share-pack`, `verify-share` |

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Motor | `crates/winlrt-archive/src/secure_share.rs` |
| Testler | `secure_share::*`, `w25_*` |
| Doğrulama | `scripts/verify-madde-w25.sh` |

## W25 %100

1. `bash scripts/verify-madde-w25.sh`
2. Main pipeline W25 adımı

**Not:** Gerçek dosya sunucusu / link dağıtımı kurum altyapısında; repo yalnızca paket formatı.
