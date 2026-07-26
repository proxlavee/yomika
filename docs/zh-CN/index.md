---
title: 概览
social_title: Yomika 中文文档
description: Yomika 是一款本地优先的漫画翻译工具，支持 OCR、图像修复、本地与远程 LLM、Web UI 以及 MCP 自动化。
hide:
  - navigation
  - toc
---

<style>
  .md-content__button {
    display: none;
  }

  .ym-home {
    --ym-bg: var(--md-default-bg-color);
    --ym-panel: color-mix(in srgb, var(--md-default-bg-color) 99.2%, var(--md-primary-fg-color) 0.8%);
    --ym-panel-strong: color-mix(in srgb, var(--md-default-bg-color) 99.6%, var(--md-primary-fg-color) 0.4%);
    --ym-panel-border: color-mix(in srgb, var(--md-default-fg-color--lightest) 92%, var(--md-primary-fg-color) 8%);
    --ym-text: var(--md-default-fg-color);
    --ym-muted: var(--md-default-fg-color--light);
    --ym-pink: var(--md-primary-fg-color);
    --ym-pink-ink: color-mix(in srgb, var(--ym-pink) 58%, var(--ym-text));
    color: var(--ym-text);
  }

  .ym-home,
  .ym-home * {
    box-sizing: border-box;
  }

  .ym-home {
    background: var(--ym-bg);
    color: var(--ym-text);
    padding: 0.5rem 0 2.5rem;
  }

  .ym-home a {
    color: inherit;
    text-decoration: none;
  }

  .ym-home h1,
  .ym-home h2,
  .ym-home h3,
  .ym-home p,
  .ym-home pre {
    margin: 0;
  }

  .ym-shell {
    width: min(100%, 60rem);
    margin: 0 auto;
    padding: 0;
  }

  .ym-announce-wrap {
    display: flex;
    justify-content: center;
  }

  .ym-announce {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-wrap: wrap;
    gap: 0.45rem;
    margin: 0;
    width: auto;
    max-width: 100%;
    padding: 0.5rem 0.72rem;
    border: 1px solid color-mix(in srgb, var(--ym-pink) 10%, var(--ym-panel-border));
    border-radius: 0.75rem;
    background: color-mix(in srgb, var(--ym-pink) 2%, var(--ym-bg));
    color: var(--ym-text);
    text-align: center;
    font-size: 0.74rem;
    font-weight: 700;
    line-height: 1.3;
  }

  .ym-announce__token {
    display: inline-flex;
    align-items: center;
    padding: 0.16rem 0.4rem;
    border-radius: 999px;
    border: 1px solid color-mix(in srgb, var(--ym-pink) 12%, var(--ym-panel-border));
    background: color-mix(in srgb, var(--ym-pink) 4%, var(--ym-bg));
    color: var(--ym-pink-ink);
    font-size: 0.68rem;
    font-weight: 800;
  }

  .ym-announce__copy {
    color: var(--ym-muted);
    font-weight: 700;
  }

  .ym-download-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 2.65rem;
    padding: 0.62rem 1rem;
    border: 1px solid color-mix(in srgb, var(--ym-pink) 18%, var(--ym-panel-border));
    border-radius: 0.65rem;
    background: color-mix(in srgb, var(--ym-pink) 10%, var(--ym-bg));
    color: var(--ym-pink-ink);
    font-size: 0.88rem;
    font-weight: 800;
    box-shadow: none;
  }

  .ym-hero {
    padding: 0.8rem 0 0;
  }

  .ym-hero__copy {
    display: grid;
    justify-items: center;
    gap: 0.9rem;
    padding: 2.6rem 0 2.1rem;
    text-align: center;
  }

  .ym-hero__copy h1 {
    max-width: none;
    font-size: clamp(2.2rem, 4.4vw, 3.45rem);
    font-weight: 900;
    line-height: 1;
    letter-spacing: -0.07em;
    text-wrap: balance;
  }

  .ym-hero__lede {
    max-width: 43rem;
    color: var(--ym-muted);
    font-size: clamp(0.98rem, 1.35vw, 1.08rem);
    line-height: 1.62;
  }

  .ym-hero__model-row {
    display: grid;
    justify-items: center;
    gap: 0.55rem;
    margin-top: -0.1rem;
  }

  .ym-hero__model-label {
    color: var(--ym-muted);
    font-size: 0.82rem;
    font-weight: 700;
    line-height: 1.4;
  }

  .ym-hero__models {
    justify-content: center;
    margin-top: 0;
  }

  .ym-download-hero {
    display: grid;
    justify-items: center;
    gap: 0.55rem;
    margin-top: 0.85rem;
  }

  .ym-download-hero .ym-download-button {
    min-width: 14.6rem;
    border-radius: 0.7rem;
    font-size: 0.9rem;
    padding-inline: 1.05rem;
  }

  .ym-download-hero__subtext {
    color: var(--ym-muted);
    font-size: 0.84rem;
    line-height: 1.5;
  }

  .ym-shot {
    margin: 0.8rem auto 0;
    width: 100%;
  }

  .ym-shot__frame {
    overflow: hidden;
    padding: 0.8rem;
    border: 1px solid color-mix(in srgb, var(--ym-panel-border) 92%, transparent);
    border-radius: 1.15rem;
    background: var(--ym-panel-strong);
    box-shadow: none;
  }

  .ym-shot img {
    display: block;
    width: 100%;
    height: auto;
    border: 1px solid color-mix(in srgb, var(--ym-panel-border) 88%, transparent);
    border-radius: 0.8rem;
  }

  .ym-section {
    padding: 3.2rem 0 0;
  }

  .ym-kicker {
    color: color-mix(in srgb, var(--ym-pink) 40%, var(--ym-text));
    font-size: 0.68rem;
    font-weight: 800;
    letter-spacing: 0.12em;
    text-transform: uppercase;
  }

  .ym-section__header {
    display: grid;
    gap: 0.9rem;
    max-width: 47rem;
  }

  .ym-section__header h2 {
    font-size: clamp(1.5rem, 2.5vw, 2rem);
    font-weight: 800;
    line-height: 1.1;
    letter-spacing: -0.06em;
  }

  .ym-section__header p {
    color: var(--ym-muted);
    font-size: 0.96rem;
    line-height: 1.62;
  }

  .ym-command-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 1rem;
    margin-top: 2rem;
  }

  .ym-command-card,
  .ym-resource-panel {
    border: 1px solid var(--ym-panel-border);
    border-radius: 1rem;
    background: var(--ym-panel);
    box-shadow: none;
  }

  .ym-command-card {
    padding: 1.2rem;
  }

  .ym-command-card__title {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    color: var(--ym-text);
    font-size: 0.88rem;
    font-weight: 800;
  }

  .ym-command-card__copy {
    margin-top: 0.55rem;
    color: var(--ym-muted);
    font-size: 0.84rem;
    line-height: 1.55;
  }

  .ym-command-card pre {
    overflow-x: auto;
    margin-top: 0.9rem;
    padding: 1rem 1.05rem;
    border: 1px solid color-mix(in srgb, var(--ym-panel-border) 88%, transparent);
    border-radius: 0.8rem;
    background: var(--ym-panel-strong);
    color: var(--ym-text);
    font-family: var(--md-code-font);
    font-size: 0.8rem;
    line-height: 1.6;
  }

  .ym-chip-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.55rem;
    margin-top: 0.95rem;
  }

  .ym-chip {
    display: inline-flex;
    align-items: center;
    padding: 0.35rem 0.6rem;
    border: 1px solid color-mix(in srgb, var(--ym-panel-border) 92%, transparent);
    border-radius: 999px;
    background: var(--ym-panel-strong);
    color: color-mix(in srgb, var(--ym-text) 84%, var(--ym-muted));
    font-size: 0.76rem;
    font-weight: 700;
    line-height: 1;
  }

  .ym-hero__models .ym-chip {
    background: color-mix(in srgb, var(--ym-pink) 3%, var(--ym-bg));
    border-color: color-mix(in srgb, var(--ym-pink) 10%, var(--ym-panel-border));
    color: var(--ym-text);
  }

  .ym-dev {
    padding-top: 3.8rem;
  }

  .ym-mcp-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 1rem;
    margin-top: 2rem;
  }

  .ym-mcp-card {
    display: grid;
    gap: 0.65rem;
    padding: 1.2rem;
    border: 1px solid var(--ym-panel-border);
    border-radius: 1rem;
    background: var(--ym-panel);
    box-shadow: none;
  }

  .ym-mcp-card h3 {
    font-size: 0.9rem;
    font-weight: 800;
    line-height: 1.3;
  }

  .ym-mcp-card p {
    color: var(--ym-muted);
    font-size: 0.84rem;
    line-height: 1.6;
  }

  .ym-dev__lead {
    display: grid;
    justify-items: center;
    gap: 1rem;
    text-align: center;
  }

  .ym-dev__lead img {
    width: 7rem;
    height: 7rem;
    object-fit: contain;
  }

  .ym-dev__lead h2 {
    font-size: clamp(1.55rem, 2.6vw, 2rem);
    font-weight: 800;
    line-height: 1.04;
    letter-spacing: -0.05em;
  }

  .ym-dev__lead p {
    max-width: 42rem;
    color: var(--ym-muted);
    font-size: 0.92rem;
    line-height: 1.65;
  }

  .ym-resource-panel {
    margin-top: 2rem;
    padding: 1.5rem;
  }

  .ym-resource-panel__grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 1rem;
  }

  .ym-resource-card {
    display: grid;
    gap: 0.8rem;
    padding: 0.65rem;
  }

  .ym-resource-card__eyebrow {
    color: color-mix(in srgb, var(--ym-pink) 42%, var(--ym-text));
    font-size: 0.76rem;
    font-weight: 800;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .ym-resource-card__copy {
    color: var(--ym-muted);
    font-size: 0.84rem;
    line-height: 1.55;
  }

  .ym-resource-card pre {
    overflow-x: auto;
    padding: 1rem;
    border: 1px solid color-mix(in srgb, var(--ym-panel-border) 88%, transparent);
    border-radius: 0.8rem;
    background: var(--ym-panel-strong);
    font-family: var(--md-code-font);
    font-size: 0.8rem;
    line-height: 1.6;
  }

  @media screen and (max-width: 76rem) {
    .ym-command-grid,
    .ym-mcp-grid,
    .ym-resource-panel__grid {
      grid-template-columns: 1fr;
    }
  }

  @media screen and (max-width: 56rem) {
    .ym-announce {
      gap: 0.35rem;
      padding: 0.45rem 0.65rem;
      font-size: 0.68rem;
    }

    .ym-hero__copy {
      padding-top: 2.1rem;
      padding-bottom: 1.7rem;
    }

    .ym-hero__copy h1 {
      font-size: clamp(1.9rem, 9vw, 2.6rem);
    }

    .ym-hero__lede {
      font-size: 0.92rem;
      line-height: 1.6;
    }

    .ym-download-hero .ym-download-button,
    .ym-download-button {
      width: 100%;
      min-width: 0;
    }

    .ym-shot__frame {
      padding: 0.55rem;
    }

    .ym-dev__lead img {
      width: 6.4rem;
      height: 6.4rem;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .ym-download-button {
      transition: none;
    }
  }
</style>

<div class="ym-home">
  <section class="ym-hero">
    <div class="ym-shell">
      <div class="ym-announce-wrap">
        <div class="ym-announce">
          <span>新功能：</span>
          <span class="ym-announce__token">基于 llama.cpp 的模型推理</span>
          <span class="ym-announce__copy">
            可在本地运行 GGUF 模型，并支持 CUDA、Vulkan 或 Metal 加速。
          </span>
        </div>
      </div>

      <div class="ym-hero__copy">
        <h1>使用本地优先的生产流水线翻译并排版漫画。</h1>
        <p class="ym-hero__lede">
          Yomika 在 Windows、macOS 和 Linux 上提供 OCR、清理、翻译、校对与导出流程。
          内置视觉管线和下载的 LLM 可在设备上运行，也可以按需选择远程翻译提供商。
        </p>
        <div class="ym-hero__model-row">
          <div class="ym-hero__model-label">内置本地模型包括</div>
          <div class="ym-chip-list ym-hero__models">
            <span class="ym-chip">sakura</span>
            <span class="ym-chip">vntl-llama3</span>
            <span class="ym-chip">hunyuan</span>
            <span class="ym-chip">lfm2</span>
          </div>
        </div>
        <div class="ym-download-hero">
          <a class="ym-download-button" href="how-to/build-from-source.md">
            从源码构建
          </a>
          <div class="ym-download-hero__subtext">
            Yomika 免费且开源。
          </div>
        </div>
      </div>
    </div>

    <div class="ym-shot">
      <div class="ym-shell">
        <div class="ym-shot__frame">
          <img src="assets/Yomika_Screenshot.png" alt="Yomika 编辑器" />
        </div>
      </div>
    </div>
  </section>

  <section class="ym-section">
    <div class="ym-shell">
      <div class="ym-section__header">
        <div class="ym-kicker">无界面部署</div>
        <h2>当你需要本地 Web UI 或可脚本化的页面流水线时，无需打开桌面窗口也能运行 Yomika。</h2>
        <p>
          桌面应用是主要使用方式，但同一套运行时也可以无界面运行。它适合在另一台机器上通过浏览器访问、
          执行可重复的批量翻译，或搭建仍然依赖 Yomika 页面感知流水线的本地自动化。
        </p>
      </div>

      <div class="ym-command-grid">
        <div class="ym-command-card">
          <div class="ym-command-card__title">Headless 模式</div>
          <div class="ym-command-card__copy">
            启动 Yomika 时不打开桌面窗口，并在固定本地端口上通过浏览器会话继续使用同一套翻译运行时。
          </div>
          <pre><code># macOS / Linux
yomika --port 4000 --headless

# Windows
yomika.exe --port 4000 --headless</code></pre>
        </div>
        <div class="ym-command-card">
          <div class="ym-command-card__title">Headless 适用场景</div>
          <div class="ym-command-card__copy">
            当你需要把现有桌面工作流换成更容易脚本化、调度或暴露给其他本地工具的形式时，就适合使用它。
          </div>
          <div class="ym-chip-list">
            <span class="ym-chip">本地 Web UI</span>
            <span class="ym-chip">批处理任务</span>
            <span class="ym-chip">脚本</span>
            <span class="ym-chip">远程桌面主机</span>
          </div>
        </div>
      </div>
    </div>
  </section>

  <section class="ym-section">
    <div class="ym-shell">
      <div class="ym-section__header">
        <div class="ym-kicker">MCP 集成</div>
        <h2>通过 MCP，让代理驱动同一套本地 Yomika 运行时。</h2>
        <p>
          Yomika 内置 MCP 支持，因此桌面编辑、Headless 模式和代理工作流都可以接入同一套本地翻译运行时，
          而不是拆成几套彼此割裂的系统。
        </p>
      </div>

      <div class="ym-mcp-grid">
        <div class="ym-mcp-card">
          <h3>一套运行时，多个入口</h3>
          <p>
            同一套页面流水线既可以服务桌面 UI，也可以服务 Headless Web UI 和 MCP 工具，
            因此自动化流程不会偏离 Yomika 在正常编辑会话中的实际行为。
          </p>
        </div>
        <div class="ym-mcp-card">
          <h3>适合代理的翻译任务</h3>
          <p>
            你可以用代理处理批量翻译、校对循环、导出以及辅助工具，只要它们需要访问 OCR、清理、
            翻译和页面级输出即可。
          </p>
        </div>
      </div>
    </div>
  </section>

  <section class="ym-dev">
    <div class="ym-shell">
      <div class="ym-dev__lead">
        <img src="assets/Yomika_Logo.png" alt="Yomika 标志" />
        <div class="ym-kicker">对开发者友好</div>
        <h2>在本地构建，并把同一套桌面运行时接入你自己的工具链。</h2>
        <p>
          Yomika 易于开发，也易于集成：使用 Bun 和 Rust 从源码构建，复用稳定的运行时参数，
          并在需要本地自动化时直接接入 Headless 模式或 MCP。
        </p>
      </div>

      <div class="ym-resource-panel">
        <div class="ym-resource-panel__grid">
          <div class="ym-resource-card">
            <div class="ym-resource-card__eyebrow">构建</div>
            <div class="ym-resource-card__copy">
              使用与项目相同的 Bun 和 Rust 工具链，从源码构建桌面应用。
            </div>
            <pre><code>bun install --frozen-lockfile
bun run build</code></pre>
          </div>
          <div class="ym-resource-card">
            <div class="ym-resource-card__eyebrow">运行参数</div>
            <div class="ym-resource-card__copy">
              桌面二进制提供一小组对本地部署和自动化很实用的参数，无需再单独搭建一个后端服务。
            </div>
            <div class="ym-chip-list">
              <span class="ym-chip">--headless</span>
              <span class="ym-chip">--port</span>
              <span class="ym-chip">--download</span>
              <span class="ym-chip">--cpu</span>
            </div>
          </div>
          <div class="ym-resource-card">
            <div class="ym-resource-card__eyebrow">自动化</div>
            <div class="ym-resource-card__copy">
              当 Yomika 需要参与更大的本地工作流时，可以在 Headless 模式或通过 MCP 复用同一套页面流水线。
            </div>
            <div class="ym-chip-list">
              <span class="ym-chip">桌面应用</span>
              <span class="ym-chip">Headless 模式</span>
              <span class="ym-chip">本地 Web UI</span>
              <span class="ym-chip">MCP 代理工作流</span>
              <span class="ym-chip">本地集成</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</div>
