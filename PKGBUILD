# Maintainer: Amar L. <amar.lakshya@xaviers.edu.in>
pkgname=workstation
pkgver=0.1.0
pkgrel=1
epoch=
pkgdesc="Rust-based commandline tool to help at your workstation."
arch=('x86_64')
url="https://github.com/amar-laksh/workstation"
license=('GPL')
depends=('opencv2','rustup')
makedepends=('git','cmake')
source=("git+ssh://git@github.com/amar-laksh/workstation.git")
md5sums=('SKIP')


prepare() {
	cd "$pkgname-$pkgver"
	make build
}

build() {
	cd "$pkgname-$pkgver"
	make
}

package() {
	cd "$pkgname-$pkgver"
	make install
}
