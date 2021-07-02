#!/usr/bin/env sh
set -e -x
cargo +nightly build --release -p realis-cli --target wasm32-unknown-unknown --no-default-features --features browser -Z features=itarget
wasm-bindgen ../../../../target/wasm32-unknown-unknown/release/realis_cli.wasm --out-dir pkg --target web
python -m http.server 8000
