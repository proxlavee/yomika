<p align="center"><img src="yomika.png" alt="Yomika logo" width="240"></p>

<h1 align="center">Yomika</h1>

<p align="center">A local-first workspace for translating, cleaning, typesetting, and exporting manga.</p>

<p align="center">
<a href="https://github.com/proxlavee/yomika/actions/workflows/build.yml"><img alt="Build" src="https://github.com/proxlavee/yomika/actions/workflows/build.yml/badge.svg?branch=main"></a>
<a href="https://github.com/proxlavee/yomika/actions/workflows/test.yml"><img alt="Test" src="https://github.com/proxlavee/yomika/actions/workflows/test.yml/badge.svg?branch=main"></a>
<a href="https://github.com/proxlavee/yomika/actions/workflows/lint.yml"><img alt="Lint" src="https://github.com/proxlavee/yomika/actions/workflows/lint.yml/badge.svg?branch=main"></a>
<a href="https://github.com/proxlavee/yomika/actions/workflows/docs.yml"><img alt="Documentation" src="https://github.com/proxlavee/yomika/actions/workflows/docs.yml/badge.svg?branch=main"></a>
<a href="LICENSE"><img alt="GPL-3.0 license" src="https://img.shields.io/github/license/proxlavee/yomika"></a>
</p>

<p align="center">
<a href="https://proxlavee.github.io/yomika/">Documentation</a> ·
<a href="https://proxlavee.github.io/yomika/tutorials/translate-your-first-page/">First-page tutorial</a> ·
<a href="https://github.com/proxlavee/yomika/issues">Issues</a> ·
<a href="https://github.com/proxlavee/yomika/discussions">Discussions</a>
</p>

<p align="center">
<a href="README.md">English</a> |
<a href="https://proxlavee.github.io/yomika/ja-JP/">日本語</a> |
<a href="https://proxlavee.github.io/yomika/zh-CN/">简体中文</a> |
<a href="https://proxlavee.github.io/yomika/pt-BR/">Português (Brasil)</a> |
<a href="docs/tr-TR/README.md">Türkçe kurulum</a>
</p>

Yomika combines text and speech-bubble detection, OCR, inpainting, translation,
review, typesetting, and export in one page-aware desktop app. The vision
pipeline and downloaded translation models can run on your device. Hosted
providers and Codex are optional and send data only when you choose those
workflows.

![Yomika editor](docs/en-US/assets/Yomika_Screenshot_en.png)

## What Yomika Does

- Processes individual pages or multi-page projects through a staged pipeline
- Runs local vision models with [candle](https://github.com/huggingface/candle)
  and local GGUF language models with [llama.cpp](https://github.com/ggml-org/llama.cpp)
- Supports local, hosted, machine-translation, and OpenAI-compatible providers
- Renders vertical CJK, right-to-left text, font fallback, strokes, and effects
- Uses Google Fonts plus installed OpenType, TrueType, and variable fonts
- Exports rendered images, editable layered PSD files, and Yomika project archives
- Exposes the same runtime through the desktop UI, headless Web UI, HTTP API, and MCP
- Separates model downloads from loading, with cancellation and storage controls

The desktop shell uses [Tauri](https://tauri.app/) and Rust; its embedded
interface is built with [Next.js](https://nextjs.org/).

## Installation

### Windows portable release

Download either the portable `.exe` or `.zip` from
[GitHub Releases](https://github.com/proxlavee/yomika/releases/latest). The ZIP
contains the same executable; extract it anywhere you can write files, then run
`Yomika-<version>-windows-x64.exe`. Yomika does not use an installer.

### Source Build Requirements

- [Rust](https://www.rust-lang.org/tools/install) 1.95 or later
- [Bun](https://bun.sh/) 1.0 or later
- LLVM/Clang with a usable `libclang` shared library
- Platform-specific [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)
- CUDA Toolkit 13.0 for the default Windows and Linux CUDA build, or Apple
  Silicon for the macOS Metal build

### Build From Source

```bash
git clone https://github.com/proxlavee/yomika.git
cd yomika
bun install --frozen-lockfile
bun run build
```

The binary is written to `target/release/yomika` or
`target/release/yomika.exe`. First launch initializes runtime libraries and
downloads the default vision/OCR models. Optional local translation models use
an explicit **Download** action and can be cancelled before you choose **Load**.

See [Build From Source](docs/en-US/how-to/build-from-source.md) for Windows,
Linux, macOS, and WSL notes.

Yomika checks GitHub for a newer release at startup and from **Settings →
About**. An available-update notice opens the Releases page; the app never
downloads or installs application updates automatically.

## First Page

1. Import one or more PNG, JPEG, or WebP pages.
2. Run **Detect → OCR → Inpaint → Translate → Render**.
3. Review text blocks, repair masks, and adjust the lettering.
4. Export a finished image or layered PSD.

Read [Translate Your First Page](docs/en-US/tutorials/translate-your-first-page.md)
for the complete walkthrough.

## Usage

### Hotkeys

Canvas:

- Mouse Wheel: Zoom in/out around the pointer
- Hand tool + Drag: Pan the canvas
- <kbd>Ctrl</kbd> + Drag: Pan the canvas

Tools:

- <kbd>V</kbd>: Select tool
- <kbd>M</kbd>: Block tool
- <kbd>B</kbd>: Brush tool
- <kbd>E</kbd>: Eraser tool
- <kbd>R</kbd>: Repair Brush tool
- <kbd>[</kbd> / <kbd>]</kbd>: Decrease / increase brush size

History and selection:

- <kbd>Ctrl</kbd> + <kbd>Z</kbd> / <kbd>Cmd</kbd> + <kbd>Z</kbd>: Undo
- <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Z</kbd> / <kbd>Cmd</kbd> + <kbd>Shift</kbd> + <kbd>Z</kbd>: Redo
- <kbd>Ctrl</kbd> + <kbd>A</kbd> / <kbd>Cmd</kbd> + <kbd>A</kbd>: Select all text blocks on the current page

For the full list and customization details, see [Keyboard Shortcuts](docs/en-US/reference/keyboard-shortcuts.md).

## Desktop, Headless, and MCP

Run without the desktop window:

```bash
# macOS / Linux
yomika --headless --port 4000

# Windows
yomika.exe --headless --port 4000
```

Open `http://127.0.0.1:4000/` for the Web UI, use
`http://127.0.0.1:4000/api/v1` for the HTTP API, or connect an MCP client to
`http://127.0.0.1:4000/mcp`. See
[Run GUI, Headless, and MCP Modes](docs/en-US/how-to/run-gui-headless-and-mcp.md).

## Models, Providers, and Acceleration

Yomika downloads required detection, OCR, inpainting, and font-analysis models
on demand. Active downloads can be cancelled, and completed model files can be
deleted or downloaded again from **Settings → Runtime**. The model library can
use its default app-data location or a folder you choose. Translation can use
a local GGUF model or a configured OpenAI,
Gemini, Claude, DeepSeek, DeepL, Google Cloud Translation, Caiyun, LM Studio,
OpenRouter, or other compatible endpoint. API keys are stored through the
platform credential store.

Codex image generation is a separate opt-in workflow that sends the source
page and prompt to the ChatGPT Codex backend. Use the staged local pipeline
when pages must remain on-device.

| Backend | Platforms | Notes |
| --- | --- | --- |
| CUDA | Windows, Linux | Main NVIDIA path for the local pipeline |
| ZLUDA | Windows | Experimental AMD path; requires AMD HIP SDK |
| Metal | Apple Silicon | Native macOS acceleration |
| Vulkan | Windows, Linux | Primarily OCR and local LLM inference |
| CPU | All | Force at runtime with `--cpu` |

See [Models and Providers](docs/en-US/explanation/models-and-providers.md) and
[Acceleration and Runtime](docs/en-US/explanation/acceleration-and-runtime.md)
for supported models, provider setup, and fallback behavior.

## Export and Project Files

- Rendered export produces a finished, flattened page.
- PSD export preserves editable text and helper layers for manual cleanup.
- `.ymk` archives move complete Yomika projects between installations.

See [Export Pages and Manage Projects](docs/en-US/how-to/export-and-manage-projects.md).

## Development

```bash
bun install --frozen-lockfile
bun run dev
```

Before opening a pull request, run the complete verification suite:

```bash
bun run verify
```

While iterating, use `bun run verify:rust` or `bun run verify:ui` to check one
side of the workspace.

Read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting changes.

## Troubleshooting

Use `yomika --debug` (or `yomika.exe --debug`) for detailed startup,
download, GPU, and model logs. The
[troubleshooting guide](docs/en-US/how-to/troubleshooting.md) covers first-run
downloads, CUDA fallback, source-build failures, headless access, and exports.

## Contributors

Thanks to everyone helping improve Yomika.

<a href="https://github.com/proxlavee/yomika/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=proxlavee/yomika" alt="Yomika contributors" />
</a>

## License

Yomika is licensed under the [GNU General Public License v3.0](LICENSE).
