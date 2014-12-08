#!/bin/sh

echo "Enter your password to install termbox"
sudo -v

echo "Fetching termbox source"
git clone https://github.com/nsf/termbox
cd termbox

echo "Configuring and installing termbox"
./waf configure --prefix=/usr
./waf
sudo ./waf install

echo "Cleaning up"
cd ../ && rm -rf termbox
