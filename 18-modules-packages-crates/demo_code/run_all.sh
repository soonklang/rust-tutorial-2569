#!/bin/bash
# รัน: bash run_all.sh   (ต้องติดตั้ง Rust ก่อน)
mkdir -p out
for f in single/*.rs; do
  n=$(basename "$f" .rs)
  echo "=============== $n ==============="
  if rustc "$f" -o "out/$n" 2>&1; then ./out/$n; else echo "[compile error ตามที่คาดไว้ถ้าเป็นไฟล์ _incorrect / _trap]"; fi
done
for p in m3_incorrect m3_correct thai_greeter; do
  echo "=============== $p ==============="
  (cd $p && cargo run 2>&1)
done
