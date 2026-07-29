# Yomika — Türkçe Kurulum

Yomika; manga sayfalarında metin ve konuşma balonu tespiti, OCR, temizleme,
çeviri, dizgi ve dışa aktarma işlemlerini tek bir masaüstü uygulamasında
birleştirir.

[Ana README](../../README.md) ·
[İngilizce belgeler](https://proxlavee.github.io/yomika/) ·
[Sürümleri indir](https://github.com/proxlavee/yomika/releases/latest) ·
[Sorun bildir](https://github.com/proxlavee/yomika/issues)

## Windows Taşınabilir Sürüm

[GitHub Releases](https://github.com/proxlavee/yomika/releases/latest)
sayfasından en güncel taşınabilir `.exe` veya `.zip` dosyasını indirin. ZIP
dosyası aynı çalıştırılabilir dosyayı içerir; istediğiniz yazılabilir klasöre
çıkarıp `Yomika-<sürüm>-windows-x64.exe` dosyasını çalıştırın. Yomika kurulum
sihirbazı kullanmaz.

## Desteklenen Derleme Yolları

| Platform | Varsayılan hızlandırma |
| --- | --- |
| Windows | CUDA |
| Linux ve WSL | CUDA |
| Apple Silicon macOS | Metal |

Windows ve Linux için standart masaüstü derlemesi CUDA özelliğini etkinleştirir.
`--cpu` yalnızca çalışma zamanı seçeneğidir; varsayılan kaynak derlemesindeki
CUDA gereksinimini kaldırmaz.

## Gereksinimler

- [Git](https://git-scm.com/)
- [Rust](https://www.rust-lang.org/tools/install) 1.95 veya üzeri
- [Bun](https://bun.sh/) 1.0 veya üzeri
- LLVM/Clang ve erişilebilir bir `libclang` ortak kitaplığı
- İlk çalıştırmada modelleri indirebilmek için internet bağlantısı ve yeterli disk alanı

### Windows

- Visual Studio C++ Build Tools
- Varsayılan CUDA derlemesi için CUDA Toolkit 13.0
- Güncel bir NVIDIA sürücüsü

Depo içindeki `scripts/dev.ts`, `nvcc` ve `cl.exe` yollarını otomatik
bulmayı dener.

### Ubuntu, Debian ve WSL

Linux CI ortamında kullanılan Tauri paketlerini kurun:

```bash
sudo apt update
sudo apt install --no-install-recommends \
  libwebkit2gtk-4.1-dev \
  build-essential \
  libclang-dev \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

Ayrıca varsayılan Linux derlemesi için CUDA Toolkit 13.0 gerekir. Bu paketler
Linux CI derleme bağımlılıklarını karşılar; WSLg penceresi ve GPU geçişi
dağıtıma ve Windows yapılandırmasına bağlıdır. Derleme geçtiği hâlde masaüstü
penceresi veya GPU çalışmıyorsa sonucu yerel Windows ortamında da doğrulayın.

### Apple Silicon macOS

Xcode komut satırı araçlarını kurun:

```bash
xcode-select --install
```

Yomika, Apple Silicon üzerinde Metal özelliğiyle derlenir.

## Kaynak Koddan Derleme

```bash
git clone https://github.com/proxlavee/yomika.git
cd yomika
bun install --frozen-lockfile
bun run build
```

Oluşan çalıştırılabilir dosya:

- Windows: `target/release/yomika.exe`
- Linux/macOS: `target/release/yomika`

`bun run build`, deponun platforma özel ayarlarını kullanan önerilen
komuttur. Doğrudan `cargo build` çağırmak bu Tauri akışını atlar.

## İlk Çalıştırma

İlk açılışta Yomika:

- yerel çıkarım için gereken çalışma zamanı kitaplıklarını hazırlar;
- varsayılan tespit, OCR ve görüntü modellerini indirir;
- isteğe bağlı yerel çeviri modellerini siz **İndir** seçeneğini kullandığınızda
  indirir.

Model indirmeleri ilerlemeyi gösterir, iptal edilebilir ve tamamlandığında bir
bildirim oluşturur. **Ayarlar > Çalışma Zamanı** bölümünden model klasörünü
değiştirebilir, geçici indirme önbelleğini temizleyebilir ve indirilen modelleri
silebilir veya yeniden indirebilirsiniz. İndirilen bir model daha sonra ayrı
bir **Yükle** işlemiyle belleğe alınır.

Arayüzü açmadan gerekli başlangıç dosyalarını indirmek için:

```bash
# Linux / macOS
target/release/yomika --download

# Windows
target/release/yomika.exe --download
```

## Çalıştırma Seçenekleri

```bash
# GPU yerine CPU kullan
yomika --cpu

# Masaüstü penceresi olmadan yerel Web UI çalıştır
yomika --headless --port 4000

# Ayrıntılı tanılama günlüğü
yomika --debug
```

Headless modda Web UI `http://127.0.0.1:4000/`, HTTP API
`http://127.0.0.1:4000/api/v1`, MCP uç noktası ise
`http://127.0.0.1:4000/mcp` adresindedir.

## Güncellemeler

Yomika açılışta en güncel GitHub sürümünü denetler. Denetimi **Ayarlar >
Hakkında** bölümünden elle de başlatabilirsiniz. Yeni sürüm bulunduğunda sağdaki
bildirim GitHub Releases sayfasını açar; Yomika uygulama güncellemelerini
otomatik olarak indirmez veya kurmaz.

## Sorun Giderme

- `nvcc not found`: CUDA Toolkit'i ve `PATH` ayarını kontrol edin.
- `libclang` bulunamıyor: `libclang-dev` paketini veya LLVM kurulumunu kontrol edin.
- WSL'de pencere açılmıyor: WSLg durumunu kontrol edin ve yerel Windows derlemesini deneyin.
- Normal açılış başarısız, `--cpu` çalışıyor: sorun büyük olasılıkla GPU yolundadır.
- Model indirme hatası: `--download --debug` ile tam hata metnini alın.

Daha ayrıntılı bilgi için [Kaynak Koddan Derleme](../en-US/how-to/build-from-source.md),
[Çalışma Zamanı ve Model İndirmeleri](../en-US/how-to/runtime-and-model-downloads.md)
ve [Sorun Giderme](../en-US/how-to/troubleshooting.md) sayfalarına bakın.
