#!/usr/bin/env node
/** Statik vitrin — Turhost nginx public köküne kopyalanır. */
const fs = require("fs");
const path = require("path");

const root = path.join(__dirname, "..", "public");
const out = path.join(__dirname, "..", "dist");
if (fs.existsSync(out)) {
  fs.rmSync(out, { recursive: true, force: true });
}
fs.mkdirSync(out, { recursive: true });

function copyDir(src, dest) {
  fs.mkdirSync(dest, { recursive: true });
  for (const name of fs.readdirSync(src)) {
    const s = path.join(src, name);
    const d = path.join(dest, name);
    if (fs.statSync(s).isDirectory()) {
      copyDir(s, d);
    } else {
      fs.copyFileSync(s, d);
    }
  }
}

copyDir(root, out);
console.log("OK: apps/web/dist hazır");
