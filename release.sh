#!/bin/bash

VERSION=$1

##currently only supporting cross compiling from M1 mac

# build mac M1
cargo build --target aarch64-apple-darwin --release
# build mac
cargo build --target x86_64-apple-darwin --release
# build windows - depends on brew install mingw-w64
cargo build --target x86_64-pc-windows-gnu --release
# build linux - depends on https://github.com/messense/homebrew-macos-cross-toolchains
export CC_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-gcc
export CXX_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-g++
export AR_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-ar
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc
cargo build --target x86_64-unknown-linux-gnu --release

# depends on brew install zip

zip  -j target/brewgen-$VERSION-arm64-apple-darwin.zip target/aarch64-apple-darwin/release/brewgen
zip  -j target/brewgen-$VERSION-amd64-apple-darwin.zip target/x86_64-apple-darwin/release/brewgen
zip  -j target/brewgen-$VERSION-amd64-pc-windows-gnu.zip target/x86_64-pc-windows-gnu/release/brewgen.exe
zip  -j target/brewgen-$VERSION-amd64-unknown-linux-gnu.zip target/x86_64-unknown-linux-gnu/release/brewgen

git tag $VERSION
git push origin $VERSION
# depends on brew install gh
gh release create $VERSION --title $VERSION --generate-notes target/brewgen-$VERSION-arm64-apple-darwin.zip target/brewgen-$VERSION-amd64-apple-darwin.zip  target/brewgen-$VERSION-amd64-pc-windows-gnu.zip target/brewgen-$VERSION-amd64-unknown-linux-gnu.zip
