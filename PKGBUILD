# Maintainer: WhatsUpeng <whatsupeng@163.com>
pkgname=smu
pkgver=0.1.0
pkgrel=1
pkgdesc="SMU 工具箱 - 轻量级实用工具集合"
arch=("x86_64")
url="https://github.com/StoneEpigraph/smu"
license=("MIT")
depends=("webkitgtk6" "gtk3")
makedepends=("cargo" "npm" "nodejs")
source=("$pkgname-$pkgver.tar.gz")
sha256sums=("SKIP")

build() {
    cd "$pkgname-$pkgver"
    npm install
    npm run tauri build -- --bundles none
}

package() {
    cd "$pkgname-$pkgver/src-tauri/target/release"
    install -Dm755 smu "$pkgdir/usr/bin/smu"
    
    install -Dm644 "$srcdir/$pkgname-$pkgver/src-tauri/cn.stonemind.smu.desktop" \
        "$pkgdir/usr/share/applications/cn.stonemind.smu.desktop"
    
    install -Dm644 "$srcdir/$pkgname-$pkgver/src-tauri/icons/128x128.png" \
        "$pkgdir/usr/share/pixmaps/smu.png"
    
    install -Dm644 "$srcdir/$pkgname-$pkgver/src-tauri/icons/32x32.png" \
        "$pkgdir/usr/share/icons/hicolor/32x32/apps/smu.png"
    
    install -Dm644 "$srcdir/$pkgname-$pkgver/src-tauri/icons/128x128.png" \
        "$pkgdir/usr/share/icons/hicolor/128x128/apps/smu.png"
}
