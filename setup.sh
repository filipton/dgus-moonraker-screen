#!/bin/bash

if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit
fi

cd ./serial-screen
cargo build --release
cp ./target/release/serial-screen /opt/serial-screen
cp ./serial-screen.service /etc/systemd/system/serial-screen.service

systemctl enable serial-screen.service
systemctl start serial-screen.service

echo "Done!"
