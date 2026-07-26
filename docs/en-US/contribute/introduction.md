---
title: Introduction
---

# Contributing to Yomika

Thank you for your interest in contributing to Yomika. We are building a local-first, ML-powered manga translator with a Rust backend and a Tauri/Next.js UI, and we would love your help.

## Quick Start

The fastest way to get started is through our [good first issues](https://github.com/proxlavee/yomika/contribute). These are carefully selected tasks that are a good fit for new contributors.

Need guidance? Open a [GitHub Discussion](https://github.com/proxlavee/yomika/discussions) or ask on the relevant issue.

## Ways to Contribute

We welcome and appreciate any form of contribution.

### Bug Reports

- Report pipeline failures in detection, OCR, inpainting, or translation
- Share crashes, regressions, and performance drops
- Document edge cases in rendering, PSD export, or provider integrations

### Feature Development

- Add new OCR, detection, inpainting, or LLM backends
- Improve the text renderer, the HTTP API, or the MCP server
- Extend the UI with new panels, shortcuts, or workflows

### Documentation

- Improve getting-started guides and how-tos
- Add examples, screenshots, or short tutorials
- Translate content to other languages

### Testing

- Add Rust unit tests for the workspace crates
- Add Vitest coverage under `ui/tests/` and Rust integration tests under `tests/integration-tests/`
- Contribute real-world manga fixtures for OCR and detection

### Infrastructure

- Improve build and CI
- Tune model downloads, runtime caching, and acceleration paths
- Keep packaging on Windows, macOS, and Linux healthy

## Understanding the Codebase

Yomika is organized as a Rust workspace with a Tauri shell and a Next.js UI:

- **`crates/yomika/`** — Tauri desktop shell
- **`crates/yomika-app/`** — application backend and pipeline orchestration
- **`crates/yomika-core/`** — shared types, events, and utilities
- **`crates/yomika-ml/`** — detection, OCR, inpainting, and font analysis
- **`crates/yomika-llm/`** — llama.cpp bindings and LLM providers
- **`crates/yomika-renderer/`** — text shaping and rendering
- **`crates/yomika-psd/`** — layered PSD export
- **`crates/yomika-rpc/`** — HTTP API and MCP server
- **`crates/yomika-runtime/`** — runtime and model download management
- **`ui/`** — Next.js web UI
- **`tests/integration-tests/`** — Rust HTTP and application integration tests
- **`ui/tests/`** — Vitest UI and frontend unit tests
- **`docs/`** — documentation site (English, 日本語, 简体中文, Português)

## Your First Contribution

1. **Browse issues.** Look at [`good first issue`](https://github.com/proxlavee/yomika/labels/good%20first%20issue).
2. **Ask questions.** Ask for clarification in the issue or in GitHub Discussions.
3. **Start small.** Docs tweaks and focused bug fixes are the easiest to land.
4. **Read the code.** Follow the patterns already in the file you are editing.

## Community

### Communication Channels

- **[GitHub Discussions](https://github.com/proxlavee/yomika/discussions)** — design discussions and open questions
- **[GitHub Issues](https://github.com/proxlavee/yomika/issues)** — bug reports and feature requests

### AI Usage Policy

When using AI tools (including LLMs like ChatGPT, Claude, Copilot, etc.) to contribute to Yomika:

- **Please disclose AI usage** to reduce maintainer fatigue
- **You are responsible** for all AI-generated issues or PRs you submit
- **Low-quality or unreviewed submissions may be closed immediately.** Contributors remain responsible for understanding and validating every change they submit.

We encourage the use of AI tools to assist with development, but all contributions must be thoroughly reviewed and tested by the contributor before submission. AI-generated code should be understood, validated, and adapted to meet Yomika's standards.

## Next Steps

Ready to contribute? Good places to start:

- **Set up locally** — see [Getting Started](development.md)
- **Find an issue** — browse [good first issues](https://github.com/proxlavee/yomika/contribute)
- **Discuss an idea** — start a [GitHub Discussion](https://github.com/proxlavee/yomika/discussions)
- **Learn the pipeline** — read [How Yomika Works](../explanation/how-yomika-works.md) and the [Technical Deep Dive](../explanation/technical-deep-dive.md)
