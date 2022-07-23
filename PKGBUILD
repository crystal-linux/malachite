# Maintainer: Matt C     <matt[at]tar[dot]black>
# Maintainer: Michal S <michal[at]tar[dot]black>
# Developer:  Michal S <michal[at]tar[dot]black>

pkgname=malachite
pkgver=2.0.0
pkgrel=1
pkgdesc="Tool for packaging and maintaining pacman repositories"
arch=('x86_64')
url="https://github.com/crystal-linux/malachite"
license=('GPL3')
source=("git+$url")
sha256sums=('SKIP')
depends=('git' 'pacman-contrib' 'gnupg')
makedepends=('cargo')

prepare() {
    cd "$srcdir/$pkgname"
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$srcdir/$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    cd "$srcdir/$pkgname"
    find target/release \
        -maxdepth 1 \
        -executable \
        -type f \
        -exec install -Dm0755 -t "${pkgdir}/usr/bin" {} +
}