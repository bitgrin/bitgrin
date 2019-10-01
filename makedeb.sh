#!/usr/bin/env bash
dh_testroot &>/dev/null
if [ $? -eq 127 ]; then
	echo "Error: debhelper missing. Try sudo apt install debhelper.";
  exit 1;
fi
fakeroot make -f debian/rules binary
