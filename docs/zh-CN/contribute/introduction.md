---
title: 简介
---

# 参与 Yomika

感谢你对 Yomika 的兴趣。Yomika 是一个本地优先、基于 Rust 的 ML 漫画翻译工具，我们非常欢迎你加入。

## 快速开始

最快上手的方式是从 [good first issues](https://github.com/proxlavee/yomika/contribute) 选一件。我们会精心挑选适合新贡献者的任务放在那里。

需要指引？请发起 [GitHub Discussion](https://github.com/proxlavee/yomika/discussions)，或在相关 Issue 中提问。

## 贡献方式

我们欢迎任何形式的贡献。

### Bug 报告

- 检测、OCR、修复、翻译流水线的问题
- 崩溃、回归、性能下降
- 渲染、PSD 导出、Provider 集成中的边界情况

### 功能开发

- 新增 OCR、检测、修复或 LLM 后端
- 改进文本渲染器、HTTP API 或 MCP Server
- 扩展 UI 面板、快捷键与工作流

### 文档

- 完善入门指南与 How-To
- 增加示例、截图或小教程
- 翻译到其他语言

### 测试

- 为 workspace 各 crate 增加 Rust 单元测试
- 扩展 `ui/tests/` 下的 Vitest 测试和 `tests/integration-tests/` 下的 Rust 集成测试
- 贡献真实漫画页样本用于 OCR 与检测

### 基础设施

- 改进构建与 CI
- 调优模型下载、运行时缓存、加速路径
- 保持 Windows、macOS、Linux 打包健康

## 认识代码库

Yomika 是一个 Rust workspace，外壳是 Tauri，UI 是 Next.js：

- **`crates/yomika/`** — Tauri 桌面外壳
- **`crates/yomika-app/`** — 应用后端与流水线编排
- **`crates/yomika-core/`** — 共享类型、事件、工具
- **`crates/yomika-ml/`** — 检测、OCR、修复、字体分析
- **`crates/yomika-llm/`** — llama.cpp 绑定与 LLM Provider
- **`crates/yomika-renderer/`** — 文本 Shape 与渲染
- **`crates/yomika-psd/`** — 分层 PSD 导出
- **`crates/yomika-rpc/`** — HTTP API 与 MCP Server
- **`crates/yomika-runtime/`** — 运行时与模型下载管理
- **`ui/`** — Next.js Web UI
- **`tests/integration-tests/`** — Rust HTTP 与应用集成测试
- **`ui/tests/`** — Vitest UI 与前端单元测试
- **`docs/`** — 文档站 (English、日本語、简体中文、Português)

## 第一次贡献

1. **浏览 Issue** — 从 [`good first issue`](https://github.com/proxlavee/yomika/labels/good%20first%20issue) 标签开始。
2. **放心提问** — 请在相关 Issue 或 GitHub Discussions 中提问。
3. **从小做起** — 文档修订和范围明确的小修复最容易合入。
4. **先读代码** — 按所编辑文件里的既有写法来。

## 社区

### 沟通渠道

- **[GitHub Discussions](https://github.com/proxlavee/yomika/discussions)** — 设计讨论与开放问题
- **[GitHub Issues](https://github.com/proxlavee/yomika/issues)** — Bug 报告与功能请求

### AI 使用政策

在为 Yomika 贡献时使用 AI 工具 (ChatGPT、Claude、Copilot 等 LLM)：

- **请声明使用了 AI** — 减轻维护者的审核负担
- **你对提交的内容负全责** — 自己提交的 Issue 或 PR 都算你的
- **低质量或未审阅的 AI 内容会直接被关闭**
- **低质量或未经审查的提交可能会被关闭。** 贡献者须理解并验证自己提交的每项更改。

我们欢迎用 AI 辅助开发，但在提交前贡献者本人必须充分审阅并测试。AI 生成的代码要读懂、验证，并调整到符合 Yomika 的标准。

## 下一步

准备好了就从这里出发：

- **本地搭建环境** — 参见 [入门](development.md)
- **找个 Issue** — 浏览 [good first issues](https://github.com/proxlavee/yomika/contribute)
- **讨论想法** — 发起 [GitHub Discussion](https://github.com/proxlavee/yomika/discussions)
- **了解流水线** — 阅读 [Yomika 工作原理](../explanation/how-yomika-works.md) 与 [技术深入](../explanation/technical-deep-dive.md)
