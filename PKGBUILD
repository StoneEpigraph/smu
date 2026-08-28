# Maintainer: WhatsUpeng <whatsupeng@163.com>
pkgname=smu
pkgver=0.1.0
pkgrel=1
pkgdesc="SMU 工具箱 - 轻量级实用工具集合"
arch=("x86_64")
url="https://github.com/StoneEpigraph/smu"
license=("MIT")
depends=("gtk3" "libsoup3")
makedepends=("cargo" "npm" "nodejs")
optdepends=("webkitgtk6: WebKit 支持 (如可用)" "webkitgtk4.0: WebKit 支持")

build() {
    cd "src-tauri"
    npm run tauri build
}

package() {
    install -Dm755 "src-tauri/target/release/smu" \
        "$pkgdir/usr/bin/smu"
    
    install -Dm644 "src-tauri/cn.stonemind.smu.desktop" \
        "$pkgdir/usr/share/applications/cn.stonemind.smu.desktop"
    
    install -Dm644 "src-tauri/icons/128x128.png" \
        "$pkgdir/usr/share/pixmaps/smu.png"
    
    install -Dm644 "src-tauri/icons/32x32.png" \
        "$pkgdir/usr/share/icons/hicolor/32x32/apps/smu.png"
    
    install -Dm644 "src-tauri/icons/128x128.png" \
        "$pkgdir/usr/share/icons/hicolor/128x128/apps/smu.png"
}