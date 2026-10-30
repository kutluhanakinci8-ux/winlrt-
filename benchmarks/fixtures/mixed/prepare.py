#!/usr/bin/env python3
"""Deterministik mixed benchmark fixture (~15+ MiB)."""
from __future__ import annotations

import os
from pathlib import Path

data = Path(__file__).resolve().parent / "data"
data.mkdir(parents=True, exist_ok=True)
chunk = ("winlrt-bench-kpi " * 400 + "\n").encode("utf-8")
(data / "app-log.txt").write_bytes(chunk * 800)
(data / "block.bin").write_bytes(bytes(i % 256 for i in range(4 * 1024 * 1024)))
(data / "sheet.csv").write_text(
    "id,name,value\n" + "\n".join(f"{i},item{i},{i * 3}" for i in range(5000)),
    encoding="utf-8",
)
total = sum(f.stat().st_size for f in data.iterdir() if f.is_file())
print(f"Prepared {data} ({total} bytes)")
