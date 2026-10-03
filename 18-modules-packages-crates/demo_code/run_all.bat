@echo off
chcp 65001 >nul
if not exist out mkdir out
for %%f in (single\*.rs) do (
  echo =============== %%~nf ===============
  rustc "%%f" -o "out\%%~nf.exe" && "out\%%~nf.exe"
)
for %%p in (m3_incorrect m3_correct thai_greeter) do (
  echo =============== %%p ===============
  pushd %%p
  cargo run
  popd
)
