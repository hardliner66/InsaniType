#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --out-name insanitype --out-dir dist --target web target/wasm32-unknown-unknown/release/insanitype.wasm

cp index.html dist

butler push dist/ iamhardliner/insanitype:wasm
butler status iamhardliner/insanitype:wasm
