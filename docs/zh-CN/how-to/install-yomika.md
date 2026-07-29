---
title: 安装 Yomika
---

# 安装 Yomika

## 下载 Windows 版本

请从 [Yomika Releases 页面](https://github.com/proxlavee/yomika/releases/latest) 下载最新的 Windows 便携版 `.exe` 或 `.zip`。ZIP 中包含同一个可执行文件。将其解压到你可管理的文件夹，然后运行 `Yomika-<version>-windows-x64.exe`；Yomika 不使用安装程序。如需开发或自定义构建，请参阅 [从源码构建](build-from-source.md)。

## Yomika 会在本地保存什么

Yomika 是本地优先应用。除便携版可执行文件外，首次运行时还会创建用户本地数据目录，用于保存：

- `llama.cpp` 与 GPU 后端所需的运行时库
- 下载的视觉模型与 OCR 模型
- 你之后在设置中选择的可选本地翻译模型

Yomika 会把自己的文件放在 `Yomika` 应用数据根目录下，并将模型权重与程序二进制分开存放。

## 首次启动时的预期行为

首次启动时，Yomika 可能会：

- 解压或下载本地推理栈所需的运行时库
- 下载检测、分割、OCR、修复和字体估计所需的默认视觉模型
- 只有当你在模型选择器中选择 **Download** 时，才下载对应的本地翻译 LLM

这属于正常现象，耗时取决于网络与硬件。
模型下载会显示进度，可以取消，并在完成后显示通知。你可以在 **Settings > Runtime** 中更改模型库文件夹、删除已下载模型或重新下载模型。

如果你想提前把运行时依赖拉下来，可以先用 `--download` 运行一次。这个路径会初始化运行时包与默认视觉栈，然后直接退出，不打开 GUI。

## 应用更新

Yomika 启动时会检查 GitHub 上的最新发行版，也可以在 **Settings > About** 中手动检查。发现新版本时，通知会打开 Releases 页面；Yomika 不会自动下载或安装应用更新。

## GPU 加速说明

Yomika 支持：

- 支持的 NVIDIA GPU 上使用 CUDA
- Apple Silicon Mac 上使用 Metal
- Windows 与 Linux 上用 Vulkan 做 OCR 与 LLM 推理
- 所有平台都可回退到 CPU

一些实际细节值得注意：

- 检测与修复阶段最受益于 CUDA 或 Metal
- Vulkan 主要是 OCR 与本地 LLM 推理的备用 GPU 路径
- 如果 Yomika 无法确认你的 NVIDIA 驱动支持 CUDA 13.0 或更新版本，它会回退到 CPU

对于支持 CUDA 的系统，Yomika 会自行初始化所需的运行时组件，而不是要求你手动配置一堆库路径。

!!! note

    请保持 NVIDIA 驱动为较新版本。Yomika 在视觉 GPU 加速上要求驱动支持 CUDA 13.0 或更新版本，Windows 上的本地 LLM CUDA 路径还需要 CUDA 13.1+。驱动太旧时，Yomika 会自动回退到 CPU。

## 安装后下一步做什么

Yomika 成功启动后，通常接下来要决定的是：

- 使用桌面 GUI 还是 headless 模式
- 使用本地翻译模型还是远程提供商
- 导出渲染图还是分层 PSD

参见：

- [以 GUI、Headless 与 MCP 模式运行](run-gui-headless-and-mcp.md)
- [模型与提供商](../explanation/models-and-providers.md)
- [导出页面与管理项目](export-and-manage-projects.md)
- [故障排查](troubleshooting.md)

## 需要帮助？

请先在 [GitHub Issues](https://github.com/proxlavee/yomika/issues) 搜索已有报告，或创建新报告。
