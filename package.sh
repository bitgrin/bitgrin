#!/bin/bash

if [[ $# -eq 0 ]] ; then
    echo 'Please pass in version number as an argument. Example: ./package.sh 1.0.6'
    exit 1
fi

#cargo build --release
mkdir -p "releases/v$1/BitGrin/"
cp ./target/release/bitgrin "releases/v$1/BitGrin/"
cp ./bats/* "releases/v$1/BitGrin/"
"C:\Program Files\7-Zip\7z" a -r "./releases/BitGrin-Windows-10-64Bit-$1.zip" "./releases/v$1/BitGrin"

rm -rf "./releases/v$1"