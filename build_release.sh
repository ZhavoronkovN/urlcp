#!/bin/bash

version=$(cargo pkgid | grep -o '#.*' | cut -c2-10)
package_name="urlcp"

cargo build

for arch in "x86_64-pc-windows-gnu"
do
    cargo build --release --target $arch
    mkdir -p ./releases/$version/$arch
    cp ./target/$arch/release/$package_name* ./releases/$version/$arch
    rm ./releases/$version/$arch/$package_name.d
done
