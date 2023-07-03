#!/bin/bash

cwd=$(pwd)
mkdir dist
cd serial-screen

cargo build --release --target aarch64-unknown-linux-gnu
cargo build --release --target arm-unknown-linux-gnueabi
#cargo build --release --target armv7-unknown-linux-gnueabihf
cargo build --release --target x86_64-unknown-linux-gnu

cd $cwd

cp ./serial-screen/target/aarch64-unknown-linux-gnu/release/serial-screen ./dist/serial-screen-aarch64-gnu
cp ./serial-screen/target/arm-unknown-linux-gnueabi/release/serial-screen ./dist/serial-screen-arm-gnueabi
#cp ./serial-screen/target/armv7-unknown-linux-gnueabihf/release/serial-screen ./dist/serial-screen-armv7-gnueabihf
cp ./serial-screen/target/x86_64-unknown-linux-gnu/release/serial-screen ./dist/serial-screen-x86_64-gnu
