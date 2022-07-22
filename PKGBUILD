# Maintainer: Matt C <mdc028[at]bucknell[dot]edu>

pkgname=malachite
pkgver=2.0.0
pkgrel=1
pkgdesc="Tool for packaging and maintaining pacman repositories"
license=('GPL3')
arch=('any')
url="https://github.com/crystal-linux/malachite"
source=("git+$url")
sha256sums=('SKIP')
depends=('git' 'pacman-contrib' 'gnupg')
makedepends=('cargo')

build() {
    cd ${srcdir}/malachite
    cargo build --release
}

package() {
    mkdir -p $pkgdir/usr/bin
    chmod +x ${srcdir}/malachite/target/release/mlc
    cp ${srcdir}/malachite/target/release/mlc $pkgdir/usr/bin/.
}
