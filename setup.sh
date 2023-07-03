#!/bin/bash

if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit
fi

arch=$(uname -m)
download_urls=$(curl -sL https://api.github.com/repos/filipton/dgus-moonraker-screen/releases/latest | grep browser_download_url | awk -F'"browser_download_url":' '{ print $2 }' | awk -F'"' '$0=$2')

if [ "$arch" = "armv6l" ]; then
    download_url=$(echo "${download_urls}" | grep "arm")
elif [ $arch = "aarch64" ]; then
    download_url=$(echo "${download_urls}" | grep "aarch64")
elif [ $arch = "x86_64" ]; then
    download_url=$(echo "${download_urls}" | grep "x86_64")
else
    echo "Unsupported architecture: $arch"
    exit
fi

echo "Downloading $download_url"

mkdir /opt/serial-screen
curl -sLo /opt/serial-screen/serial-screen $download_url
chmod +x /opt/serial-screen/serial-screen

curl -sLo /etc/systemd/system/serial-screen.service https://raw.githubusercontent.com/filipton/dgus-moonraker-screen/master/serial-screen.service

systemctl daemon-reload
systemctl enable serial-screen.service
systemctl start serial-screen.service

echo "Done!"
