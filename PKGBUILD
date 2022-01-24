# Maintainer: Matt C <mdc028[at]bucknell[dot]edu>

pkgname=malachite
pkgver=1.0.0
pkgrel=1
pkgdesc="Crystal Linux Wallpaper Images"
arch=('any')
url="https://git.tar.black/crystal/programs/malachite"
license=()
source=()
depends=()
conflicts=()
md5sums=()

package() {

    cd ../ && cargo build --release
    mkdir -p "${pkgdir}/usr/bin"
    cp target/release/mlc "${pkgdir}/usr/bin/."

}
