Karışık boyut KPI seti (Faz 3): ~15+ MiB metin + ikili + CSV (hız KPI ölçülebilir süre).

Dosyalar `prepare.sh` ile üretilir (CI ve yerel bench öncesi):

  bash benchmarks/fixtures/mixed/prepare.sh

`bench --all` bu klasörü diğer fixture’larla birlikte ölçer.
