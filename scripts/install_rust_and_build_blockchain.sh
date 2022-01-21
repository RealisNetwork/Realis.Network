#!/bin/bash

sudo apt update && sudo apt install -y git clang curl libssl-dev llvm libudev-dev && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && source ~/.cargo/env && rustup default stable && rustup update &&  rustup update nightly && rustup target add wasm32-unknown-unknown --toolchain nightly && rustup toolchain install nightly-2021-11-18 && rustup override set nightly-2021-11-18 && rustup target add wasm32-unknown-unknown --toolchain nightly-2021-11-18 && sudo apt-get install make -y && make build
