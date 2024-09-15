#!/bin/bash

echo "verifying the toolchains..."
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
echo "toolchains are ready!"

echo "building binaries for linux..."
cargo build --release --target x86_64-unknown-linux-gnu
strip target/x86_64-unknown-linux-gnu/release/absurd
echo "linux binary size:"
du -h target/x86_64-unknown-linux-gnu/release/absurd

echo "building binaries for Windows..."
cargo build --release --target x86_64-pc-windows-gnu
strip target/x86_64-pc-windows-gnu/release/absurd.exe
echo "windows binary size:"
du -h target/x86_64-pc-windows-gnu/release/absurd.exe
