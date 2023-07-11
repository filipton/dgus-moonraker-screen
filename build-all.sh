#!/bin/bash

cwd=$(pwd)
mkdir dist
cd serial-screen

cargo build --release --target aarch64-unknown-linux-gnu
#cargo build --release --target arm-unknown-linux-gnueabi
cross build --release --target arm-unknown-linux-gnueabihf
cargo build --release --target x86_64-unknown-linux-gnu

cd $cwd

cp ./serial-screen/target/aarch64-unknown-linux-gnu/release/serial-screen ./dist/serial-screen-aarch64-gnu
cp ./serial-screen/target/arm-unknown-linux-gnueabihf/release/serial-screen ./dist/serial-screen-arm-gnueabihf
#cp ./serial-screen/target/arm-unknown-linux-gnueabi/release/serial-screen ./dist/serial-screen-arm-gnueabi
cp ./serial-screen/target/x86_64-unknown-linux-gnu/release/serial-screen ./dist/serial-screen-x86_64-gnu
