#!/bin/bash

if [[ $# -eq 0 ]] ; then
    echo 'Please pass in version number as an argument. Example: ./package.sh 1.0.6'
    exit 1
fi

cargo build --release
mkdir -p "releases/v$1/bitgrin-$1/"
cp ./target/release/bitgrin "releases/v$1/bitgrin-$1/"

# Perform build on wallet as well
cd ../bitgrin-wallet/
cargo build --release
cp target/release/bitgrin-wallet "../bitgrin/releases/v$1/bitgrin-$1/"
cd ../bitgrin

echo "$OSTYPE"

if [[ "$OSTYPE" == "linux-gnu" ]]; then
    echo 'Linux'
    cd "./releases/v$1"
    zip -r "../BitGrin-Linux-$1.zip" "./bitgrin-$1"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo 'Mac OS'
    cd "./releases/v$1"
    zip -r "../BitGrin-Mac-OS-$1.zip" "./bitgrin-$1"
else
    echo 'Windows'
    cp ./bats/* "releases/v$1/bitgrin-$1/"
    "C:\Program Files\7-Zip\7z" a -r -tzip "./releases/BitGrin-Windows-10-64Bit-$1.zip" "./releases/v$1/bitgrin-$1"
fi

rm -rf "./releases/v$1"