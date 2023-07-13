#!/bin/bash

last=$(curl -s https://api.github.com/repos/filipton/dgus-moonraker-screen/releases/latest | jq -r .tag_name)
echo "Last release: $last"

echo -n "Release version (for eg. 2.4.1): "
read version

echo "pub const VERSION: &str = \"$version\";" > ./serial-screen/src/version.rs
bash ./build-all.sh
gh release create --latest --generate-notes $version ./dist/*
