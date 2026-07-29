---
title: Install Yomika
---

# Install Yomika

## Download a release

Download the latest Windows portable `.exe` or `.zip` from the
[Yomika Releases page](https://github.com/proxlavee/yomika/releases/latest).
The ZIP contains the same executable. Extract it to a folder you control and
run `Yomika-<version>-windows-x64.exe`; there is no installer. For a development
or custom build, follow
[Build From Source](build-from-source.md).

## What Yomika stores locally

Yomika is a local-first application. The portable desktop binary is only part
of its disk footprint. The first real run also creates a per-user local data
directory for:

- runtime libraries used by llama.cpp and GPU backends
- downloaded vision and OCR models
- optional local translation models you select later

Yomika keeps its own files under a `Yomika` app-data root and stores model weights separately from the application binary.

## First launch expectations

On first run, Yomika may:

- extract or download runtime libraries required by the local inference stack
- download the default vision and OCR models used by detection, segmentation, OCR, inpainting, and font estimation
- wait to download optional local translation LLMs until you choose **Download** in the model picker

This is normal and can take a while depending on your connection and hardware.
Model downloads show progress, can be cancelled, and report completion in the
notification area. Use **Settings > Runtime** to change the model-library
folder, delete downloaded models, or download a model again.

If you want to prefetch those runtime dependencies ahead of time, run Yomika once with `--download`. That path initializes the runtime packages and default vision stack, then exits without opening the GUI.

## Application updates

Yomika checks the latest GitHub release when it starts. You can also run the
check from **Settings > About**. When a newer version exists, Yomika shows a
notification that opens the Releases page; it never downloads or installs an
application update automatically.

## GPU acceleration notes

Yomika supports:

- CUDA on supported NVIDIA GPUs
- Metal on Apple Silicon Macs
- Vulkan on Windows and Linux for OCR and LLM inference
- CPU fallback on all platforms

Some practical details matter:

- detection and inpainting benefit most from CUDA or Metal
- Vulkan is mainly the fallback GPU path for OCR and local LLM inference
- if Yomika cannot verify that your NVIDIA driver supports CUDA 13.0 or newer, it falls back to CPU

On CUDA-capable systems, Yomika bundles and initializes the runtime pieces it needs instead of requiring you to configure every library path manually.

!!! note

    Keep your NVIDIA driver up to date. Yomika requires a driver supporting CUDA 13.0 or newer for vision GPU acceleration, and CUDA 13.1+ on Windows for the local LLM CUDA path. If the driver is too old, Yomika falls back to CPU.

## After installation

Once Yomika launches successfully, the next decisions are usually:

- desktop GUI vs headless mode
- local translation model vs remote provider
- rendered export vs layered PSD export

See:

- [Run GUI, Headless, and MCP Modes](run-gui-headless-and-mcp.md)
- [Models and Providers](../explanation/models-and-providers.md)
- [Export Pages and Manage Projects](export-and-manage-projects.md)
- [Troubleshooting](troubleshooting.md)

## Need help?

Search or open a report in [GitHub Issues](https://github.com/proxlavee/yomika/issues).
