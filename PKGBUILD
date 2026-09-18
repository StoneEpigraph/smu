# Maintainer: WhatsUpeng <whatsupeng@163.com>
pkgname=smu
pkgver=0.1.0
pkgrel=2
pkgdesc="SMU 工具箱 - 轻量级实用工具集合"
arch=("x86_64")
url="https://github.com/StoneEpigraph/smu"
license=("MIT")
# Tauri v2 运行时依赖：webkit2gtk-4.1（gtk3/libsoup3 由其传递引入）
# 托盘图标依赖 libappindicator（tray-icon crate 的 libappindicator 特性）
depends=("webkit2gtk-4.1" "gtk3" "libappindicator")
makedepends=("cargo" "npm" "nodejs" "pkgconf")
options=("!lto")
source=("smu-$pkgver.tar.gz")
sha256sums=("SKIP")

prepare() {
    cd "$srcdir/$pkgname-$pkgver"
    npm ci
}

build() {
    cd "$srcdir/$pkgname-$pkgver"
    # 使用独立 target 目录，避免与用户环境的 CARGO_TARGET_DIR(~/.target) 共享缓存
    export CARGO_TARGET_DIR="$srcdir/target"
    # 只需要编译出二进制，跳过 deb/appimage 打包步骤
    npm run tauri build -- --no-bundle
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    install -Dm755 "$srcdir/target/release/smu" \
        "$pkgdir/usr/bin/smu"

    install -Dm644 "src-tauri/cn.stonemind.smu.desktop" \
        "$pkgdir/usr/share/applications/cn.stonemind.smu.desktop"

    install -Dm644 "src-tauri/icons/32x32.png" \
        "$pkgdir/usr/share/icons/hicolor/32x32/apps/smu.png"
    install -Dm644 "src-tauri/icons/128x128.png" \
        "$pkgdir/usr/share/icons/hicolor/128x128/apps/smu.png"
    install -Dm644 "src-tauri/icons/128x128@2x.png" \
        "$pkgdir/usr/share/icons/hicolor/256x256/apps/smu.png"
    install -Dm644 "src-tauri/icons/icon.png" \
        "$pkgdir/usr/share/icons/hicolor/512x512/apps/smu.png"

    if [[ -f LICENSE ]]; then
        install -Dm644 "LICENSE" \
            "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    fi
}
