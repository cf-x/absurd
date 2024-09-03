#!/bin/bash

rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu

echo "Building for Linux..."
cargo build --release --target x86_64-unknown-linux-gnu
strip target/x86_64-unknown-linux-gnu/release/absurd
echo "Linux binary size:"
du -h target/x86_64-unknown-linux-gnu/release/absurd

echo "Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu
strip target/x86_64-pc-windows-gnu/release/absurd.exe
echo "Windows binary size:"
du -h target/x86_64-pc-windows-gnu/release/absurd.exe
