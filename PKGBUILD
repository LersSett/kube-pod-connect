pkgname=kube-pod-connect
pkgver=0.0.1
pkgrel=1
pkgdesc='Simple connect to kubernetes pod'
arch=(any)
url='https://github.com/Stanislav-Lapata/kube-pod-connect'
license=(MIT)
depends=(rust)
source=(
  https://github.com/Stanislav-Lapata/$pkgname/archive/v$pkgver.tar.gz
)
sha1sums=(
)

package() {
  mkdir -p "$pkgdir/opt"
  cp -r "$srcdir/$pkgname-$pkgver" "$pkgdir/opt/kube-pod-connect"

  cd "$pkgdir/opt/kube-pod-connect"

  cargo build --release


  find "$pkgdir" -type f -exec chmod 644 {} \;
  find "$pkgdir" -mindepth 1 -type d -exec chmod 755 {} \;
}
