#!/usr/bin/env sh
# 生成 Arch 源码包(smu-<version>.tar.gz)并刷新 .SRCINFO。
# 版本号以 src-tauri/tauri.conf.json 为唯一来源，与 PKGBUILD 的 smu-$pkgver.tar.gz 对应。
set -e

cd "$(dirname "$0")/.."

VERSION=$(node -p "require('./src-tauri/tauri.conf.json').version")

git archive --format=tar.gz --prefix="smu-$VERSION/" -o "smu-$VERSION.tar.gz" HEAD
makepkg --printsrcinfo > .SRCINFO

echo "已生成 smu-$VERSION.tar.gz 与 .SRCINFO"
