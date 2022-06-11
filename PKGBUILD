# Maintainer: Matt C <mdc028[at]bucknell[dot]edu>

pkgname=malachite
pkgver=1.2.0
pkgrel=1
pkgdesc="Tool for packaging and maintaining pacman repositories"
license=('GPL3')
arch=('any')
url="https://git.tar.black/crystal/programs/malachite"
license=('Nolicense')
source=("git+https://git.tar.black/crystal/programs/malachite")
sha256sums=('SKIP')
depends=('git')
makedepends=('cargo')

build() {
    cd ${srcdir}/malachite
    cargo build --release
}

package() {
    mkdir -p $pkgdir/usr/bin
    chmod +x ${srcdir}/malachite/target/release/mlc
    cp ${srcdir}/malachite/target/release/mlc  $pkgdir/usr/bin/.
}
