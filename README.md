# AnimeciX Linux App

[![Releases](https://img.shields.io/github/v/release/Ayzrith/animecix-linux-app?style=flat-square)](https://github.com/Ayzrith/animecix-linux-app/releases/latest)
[![License: MIT](https://img.shields.io/github/license/Ayzrith/animecix-linux-app?style=flat-square)](LICENSE)

<img src="assets/hicolor/256x256/apps/tr.com.animecix.png" align="right" width="96" height="96" alt="AnimeciX Linux App">

**Linux için anime, dizi ve film izleme masaüstü uygulaması (GTK4 + libadwaita).**

Tek dosyalık taşınabilir **AppImage** olarak çalışır; yeni sürüm çıktığında
uygulama içinden kendini güncelleyebilir.

> ⚠️ Bu uygulama **gayriresmîdir** ve kişisel kullanım amaçlıdır.
> AnimeciX (animecix.net) ile resmi bir bağlantısı yoktur.

---

## Özellikler

- 🏠 Kategorilere göre anime / dizi / film listeleri
- 🔎 Hızlı arama (ana ekranda `Ctrl+S`, bölüm ekranında `/`)
- ▶️ MPV ile oynatma, otomatik tam ekran, intro/outro atlama (AniSkip)
- ⭐ Favoriler, 🏃 izleme maratonu, 🕘 geçmiş takibi
- 🎯 Akıllı kaynak seçimi: tüm kaynaklar paralel çözülür, en kalitelisi açılır;
  açılmayan kaynakta otomatik sonrakine geçilir
- 🔄 Otomatik güncelleme (AppImage sürümünde)
- 🛡️ İsteğe bağlı VPN proxy desteği (`127.0.0.1:10808` — örn. sing-box +
  ProtonVPN); proxy yoksa uygulama normal çalışır
- ⚙️ Masaüstü başlatıcı kurulumu, veri sıfırlama, değiştirilebilir kısayollar

---

## Kurulum

### AppImage (önerilen)

1. [Releases](https://github.com/Ayzrith/animecix-linux-app/releases/latest)
   sayfasından `AnimeciX-x86_64.AppImage` dosyasını indirin.
2. Çalıştırılabilir yapıp açın:

```bash
chmod +x AnimeciX-x86_64.AppImage
./AnimeciX-x86_64.AppImage
```

Masaüstü menüsüne eklemek için uygulama içinden
**Ayarlar → Masaüstü Başlatıcısını Sistemime Kur** seçeneğini kullanabilirsiniz.

### Kaynaktan derleme

Gerekenler:

- **Debian/Ubuntu:** `sudo apt install libgtk-4-dev libadwaita-1-dev mpv pkg-config`
- **Fedora:** `sudo dnf install gtk4-devel libadwaita-devel mpv pkgconf-pkg-config`
- **Arch:** `sudo pacman -S gtk4 libadwaita mpv pkgconf`
- Rust 1.74+

```bash
git clone https://github.com/Ayzrith/animecix-linux-app.git
cd animecix-linux-app
cargo build --release
./target/release/animecix
```

> Not: bu sistemde derleme için önce şunlar gerekir:
>
> ```bash
> export LIBCLANG_PATH=/usr/lib/llvm-21/lib
> export BINDGEN_EXTRA_CLANG_ARGS="--target=x86_64-unknown-linux-gnu -I/usr/lib/gcc/x86_64-linux-gnu/15/include -I/usr/include/x86_64-linux-gnu -I/usr/include"
> ```

### AppImage paketleme

```bash
bash build_appimage.sh
```

Betik sürümü otomatik artırır ve `AnimeciX-x86_64.AppImage` üretir.
`GITHUB_TOKEN` tanımlıysa GitHub Release olarak da yayınlar.

---

## Güncelleme

AppImage olarak çalışıyorsa başlangıçta yeni sürüm denetlenir:

- **Otomatik:** *Ayarlar → Güncelleme → Otomatik Güncelleme* açıksa onay kutusu
  çıkar; “Güncelle ve Yeniden Başlat” ile kurulur.
- **Elle:** *Ayarlar → Şimdi Güncelle* ile istenen an denetlenir.

---

## Klavye kısayolları

| Kısayol  | İşlev                              |
|----------|------------------------------------|
| `/`      | Bölüm ekranında hızlı bölüm arama  |
| `Ctrl+S` | Ana ekranda arama çubuğunu aç      |
| `s` / `e`| İntro / outro sonuna atla         |
| `Esc`    | Geri / aramayı kapat               |

---

## Veriler nerede tutulur?

```
~/.local/share/animecix/
~/.cache/animecix/
```

*Ayarlar → Tüm Verileri Sıfırla ve Temizle* ile sıfırlanabilir.

---

## Atıf ve lisans

Bu proje [nyx47rd/animecix](https://github.com/nyx47rd/animecix) tabanlı
gayriresmî bir çalışmadır. [MIT](LICENSE) Lisansı © 2026 nyx47rd —
lisans ve telif bildirimi korunmuştur.
