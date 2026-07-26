---
title: 概览
social_title: Yomika
description: Yomika 是一套本地优先的漫画翻译工作区，涵盖 OCR、图像修复、翻译、排版、导出与自动化。
hide:
  - navigation
  - toc
---

<div class="ym-home">
  <div class="ym-shell">
    <section class="ym-hero" aria-labelledby="ym-hero-title">
      <div class="ym-hero__grid">
        <div class="ym-hero__copy">
          <div class="ym-kicker">PAGE 01 · 本地优先的漫画制作</div>
          <h1 id="ym-hero-title">从原始漫画页到<span>完成排字。</span></h1>
          <p class="ym-hero__lede">
            Yomika 把检测、OCR、图像修复、翻译、校对、排版与导出放进同一个理解页面结构的工作区。
            内置流水线默认在本地运行，只有项目确实需要时才接入远程服务。
          </p>
          <div class="ym-actions">
            <a class="ym-button ym-button--primary" href="how-to/build-from-source.md">构建 Yomika</a>
            <a class="ym-button ym-button--secondary" href="tutorials/translate-your-first-page.md">翻译第一页</a>
          </div>
          <ul class="ym-facts" aria-label="Yomika 概要">
            <li>Windows、Linux、Apple Silicon</li>
            <li>GPL-3.0</li>
            <li>桌面、Headless、MCP</li>
          </ul>
        </div>

        <div class="ym-hero__visual">
          <div class="ym-panel-label">EDITOR / PAGE VIEW</div>
          <div class="ym-screen">
            <img src="assets/Yomika_Screenshot.png" alt="Yomika 正在编辑翻译后的漫画页面" />
          </div>
          <div class="ym-callout ym-callout--top">
            <strong>默认本地运行</strong>
            <span>视觉模型和下载的 LLM 留在设备上。</span>
          </div>
          <div class="ym-callout ym-callout--bottom">
            <strong>可编辑交付</strong>
            <span>导出渲染页面或分层 PSD 文件。</span>
          </div>
        </div>
      </div>
    </section>

    <section class="ym-workflow" aria-labelledby="ym-workflow-title">
      <div class="ym-section-heading">
        <div>
          <div class="ym-kicker">一页到底，流程不断</div>
          <h2 id="ym-workflow-title">衔接完整的汉化工作台。</h2>
        </div>
        <p>
          每个阶段直接把结果交给下一个阶段，不必在互不相关的工具之间搬运素材。
          检测框、识别文本和最终排版随时都能校对与调整。
        </p>
      </div>
      <ol class="ym-steps">
        <li class="ym-step">
          <span class="ym-step__number">01</span>
          <strong>检测</strong>
          <p>定位文本区域、对话框和页面结构。</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">02</span>
          <strong>OCR</strong>
          <p>把对白与旁白识别成可校对的 Unicode 文本。</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">03</span>
          <strong>修复</strong>
          <p>在保留画面的同时移除原文文字。</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">04</span>
          <strong>翻译</strong>
          <p>使用本地 GGUF 模型或可选的远程提供商。</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">05</span>
          <strong>排版</strong>
          <p>校对、渲染，并导出完成页面或分层 PSD。</p>
        </li>
      </ol>
    </section>

    <section class="ym-section" aria-labelledby="ym-modes-title">
      <div class="ym-section-heading">
        <div>
          <div class="ym-kicker">选择工作方式</div>
          <h2 id="ym-modes-title">一套运行时，三种入口。</h2>
        </div>
        <p>
          桌面编辑器、Headless Web UI、HTTP API 与 MCP 工具共享同一份项目状态和页面流水线。
        </p>
      </div>
      <div class="ym-mode-grid">
        <a class="ym-mode-card ym-mode-card--desktop" href="tutorials/translate-your-first-page.md">
          <span class="ym-mode-tag">桌面编辑器</span>
          <h3>细调每个对话框、蒙版与文字块。</h3>
          <p>
            导入整组页面，单独运行每个阶段，修补蒙版，调整排字，并通过导出保留可编辑结果。
          </p>
          <ul class="ym-pills">
            <li>批量项目</li>
            <li>CJK 竖排</li>
            <li>RTL 布局</li>
            <li>分层 PSD</li>
          </ul>
          <span class="ym-mode-card__arrow" aria-hidden="true">↗</span>
        </a>
        <a class="ym-mode-card" href="how-to/run-gui-headless-and-mcp.md">
          <span class="ym-mode-tag">Headless</span>
          <h3>在浏览器中打开同一个工作区。</h3>
          <p>无需桌面窗口即可运行脚本、批处理任务或固定端口的本地服务器。</p>
          <span class="ym-mode-card__arrow" aria-hidden="true">↗</span>
        </a>
        <a class="ym-mode-card" href="how-to/configure-mcp-clients.md">
          <span class="ym-mode-tag">MCP + HTTP API</span>
          <h3>让代理操作真实的项目状态。</h3>
          <p>自动执行项目任务，同时让日常编辑与代理操作保持一致。</p>
          <span class="ym-mode-card__arrow" aria-hidden="true">↗</span>
        </a>
      </div>
    </section>

    <section class="ym-privacy" aria-labelledby="ym-privacy-title">
      <div class="ym-privacy__copy">
        <div class="ym-kicker">本地优先，但不限于本地</div>
        <h2 id="ym-privacy-title">页面留在手边，云端由你选择。</h2>
        <p>
          Yomika 可以在你的设备上运行视觉栈和下载的翻译模型。
          远程 LLM、机器翻译和 Codex 流程都是明确的选项，而不是隐藏依赖。
        </p>
      </div>
      <ul class="ym-privacy__list">
        <li>
          <span class="ym-privacy__mark">A</span>
          <span><strong>本地流水线</strong>检测、OCR、清理和本地翻译都能留在设备上。</span>
        </li>
        <li>
          <span class="ym-privacy__mark">B</span>
          <span><strong>服务可控</strong>由你决定何时把文本或图像发送给已配置的服务。</span>
        </li>
        <li>
          <span class="ym-privacy__mark">C</span>
          <span><strong>实用输出</strong>保存合成图、可编辑 PSD 图层或项目归档。</span>
        </li>
      </ul>
    </section>

    <section class="ym-install" aria-labelledby="ym-install-title">
      <div class="ym-install__grid">
        <div class="ym-install__copy">
          <div class="ym-kicker">源码构建</div>
          <h2 id="ym-install-title">准备处理第一页了吗？</h2>
          <p>
            目前还没有发布预编译版本。请使用仓库提供的 Bun 包装命令，
            以便启用正确的 Tauri 与平台功能路径。
          </p>
        </div>
        <div>
          <pre><code>git clone https://github.com/proxlavee/yomika.git
cd yomika
bun install --frozen-lockfile
bun run build</code></pre>
          <div class="ym-install__links">
            <a class="ym-text-link" href="how-to/build-from-source.md">查看依赖与平台说明</a>
            <a class="ym-text-link" href="how-to/runtime-and-model-downloads.md">了解首次启动下载</a>
            <a class="ym-text-link" href="https://github.com/proxlavee/yomika">在 GitHub 查看源码</a>
          </div>
        </div>
      </div>
    </section>
  </div>
</div>
