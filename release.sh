#!/bin/sh
# This script makes release zips/tar.gzs for github
sudo rm -rf ./package gh_release ./temp

mkdir -p ./temp/linux ./temp/win

# Linux exec
echo building linux
cargo build --release --target x86_64-unknown-linux-gnu
cp ./target/x86_64-unknown-linux-gnu/release/qrgen ./temp/linux/AppRun
ldd ./temp/linux/AppRun | grep -o '/[^[:space:]]*' | awk '{system("cp "$1" ./temp/linux/")}'
cp ./qrcode.png ./temp/linux/qrcode.png
ln -s ./temp/linux/qrcode.png ./temp/linux/.DirIcon

# Windows exec
echo building windows
docker run -v `pwd`:/mnt mglolenstine/gtk4-cross:rust-gtk-4.10 /bin/bash -c "build; package"
sudo chown $(whoami):$(whoami) -R ./temp ./package
sudo mv ./package/* ./temp/win/.
sudo rmdir ./package

# Package it all up
echo packing
cp ./LICENSE.md ./temp/win/.
cp ./README.md ./temp/win/.

mkdir gh_release

echo "[Desktop Entry]
Name=QR Code Generator
Exec=AppRun
Icon=qrcode
Type=Application
Categories=Utility;" > ./temp/linux/qrgen.desktop
appimagetool ./temp/linux/ --comp xz

mv ./QR_Code_Generator-x86_64.AppImage ./gh_release/qrgen-x86_x64.AppImage
$(cd ./temp/win; zip -r ../../gh_release/win-x86_64.zip .)

rm -rf ./temp
echo "Done!"