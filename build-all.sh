#!/bin/bash

cwd=$(pwd)
mkdir builds
cd serial-screen

cargo build --release --target aarch64-unknown-linux-gnu
cargo build --release --target arm-unknown-linux-gnueabi

cd $cwd

cp ./serial-screen/target/aarch64-unknown-linux-gnu/release/serial-screen ./builds/serial-screen-aarch64
cp ./serial-screen/target/arm-unknown-linux-gnueabi/release/serial-screen ./builds/serial-screen-arm-gnueabi
