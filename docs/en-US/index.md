---
title: Overview
social_title: Yomika
description: Yomika is a local-first manga translator built in Rust with OCR, inpainting, local and remote LLM support, a Web UI, and MCP automation.
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
          <span>Now available:</span>
          <span class="ym-announce__token">llama.cpp local inference</span>
          <span class="ym-announce__copy">
            Run GGUF models locally with CUDA, Vulkan, or Metal acceleration.
          </span>
        </div>
      </div>

      <div class="ym-hero__copy">
        <h1>Translate and typeset manga with a local-first production pipeline.</h1>
        <p class="ym-hero__lede">
          Yomika handles OCR, cleanup, translation, review, and export on Windows,
          macOS, and Linux. Run the built-in vision pipeline and downloaded LLMs
          on-device, or opt into a hosted translation provider.
        </p>
        <div class="ym-hero__model-row">
          <div class="ym-hero__model-label">Local models include</div>
          <div class="ym-chip-list ym-hero__models">
            <span class="ym-chip">sakura</span>
            <span class="ym-chip">vntl-llama3</span>
            <span class="ym-chip">hunyuan</span>
            <span class="ym-chip">lfm2</span>
          </div>
        </div>
        <div class="ym-download-hero">
          <a class="ym-download-button" href="how-to/build-from-source.md">
            Build from source
          </a>
          <div class="ym-download-hero__subtext">
            Free and open source.
          </div>
        </div>
      </div>
    </div>

    <div class="ym-shot">
      <div class="ym-shell">
        <div class="ym-shot__frame">
          <img src="assets/Yomika_Screenshot_en.png" alt="Yomika editor" />
        </div>
      </div>
    </div>
  </section>

  <section class="ym-section">
    <div class="ym-shell">
      <div class="ym-section__header">
        <div class="ym-kicker">Headless deployment</div>
        <h2>Run Yomika without the desktop window when you need a local Web UI or a scriptable translation runtime.</h2>
        <p>
          The desktop app is the primary interface, but the same runtime can also run
          headless. Use it for browser-based access, repeatable batch work, or local
          automation that still depends on Yomika's page-aware pipeline.
        </p>
      </div>

      <div class="ym-command-grid">
        <div class="ym-command-card">
          <div class="ym-command-card__title">Headless mode</div>
          <div class="ym-command-card__copy">
            Start Yomika without the desktop window and keep the same translation
            runtime available through a browser session on a fixed local port.
          </div>
          <pre><code># macOS / Linux
yomika --port 4000 --headless

# Windows
yomika.exe --port 4000 --headless</code></pre>
        </div>
        <div class="ym-command-card">
          <div class="ym-command-card__title">What headless is for</div>
          <div class="ym-command-card__copy">
            Use it when you need the desktop workflow in a form that is easier to
            script, schedule, or expose to other local tools.
          </div>
          <div class="ym-chip-list">
            <span class="ym-chip">Local Web UI</span>
            <span class="ym-chip">Batch jobs</span>
            <span class="ym-chip">Scripts</span>
            <span class="ym-chip">Remote desktop host</span>
          </div>
        </div>
      </div>
    </div>
  </section>

  <section class="ym-section">
    <div class="ym-shell">
      <div class="ym-section__header">
        <div class="ym-kicker">MCP Integration</div>
        <h2>Let agents drive the same local Yomika runtime through MCP.</h2>
        <p>
          Yomika includes MCP support so the desktop UI, headless mode, and agent
          workflows all talk to the same local translation runtime instead of drifting
          into separate stacks.
        </p>
      </div>

      <div class="ym-mcp-grid">
        <div class="ym-mcp-card">
          <h3>One runtime, multiple entry points</h3>
          <p>
            The same page pipeline powers the desktop UI, the headless Web UI, and MCP
            tools, so automation stays aligned with normal editing sessions.
          </p>
        </div>
        <div class="ym-mcp-card">
          <h3>Agent-friendly translation tasks</h3>
          <p>
            Use agents for batch translation, review loops, exports, and helper tooling
            that needs access to OCR, cleanup, translation, and page-level outputs.
          </p>
        </div>
      </div>
    </div>
  </section>

  <section class="ym-dev">
    <div class="ym-shell">
      <div class="ym-dev__lead">
        <img src="assets/Yomika_Logo.png" alt="Yomika logo" />
        <div class="ym-kicker">Developer-friendly</div>
        <h2>Build from source and reuse the same runtime in your own tooling.</h2>
        <p>
          Yomika is designed to be practical to build and practical to integrate. Use
          Bun and Rust for local builds, stable runtime flags for deployment, and
          headless mode or MCP when you need automation around the app.
        </p>
      </div>

      <div class="ym-resource-panel">
        <div class="ym-resource-panel__grid">
          <div class="ym-resource-card">
            <div class="ym-resource-card__eyebrow">Build</div>
            <div class="ym-resource-card__copy">
              Build the desktop app from source with the same Bun and Rust toolchain
              used by the project.
            </div>
            <pre><code>bun install --frozen-lockfile
bun run build</code></pre>
          </div>
          <div class="ym-resource-card">
            <div class="ym-resource-card__eyebrow">Runtime flags</div>
            <div class="ym-resource-card__copy">
              The desktop binary exposes a small set of runtime flags for local
              deployment and automation without introducing a separate backend service.
            </div>
            <div class="ym-chip-list">
              <span class="ym-chip">--headless</span>
              <span class="ym-chip">--port</span>
              <span class="ym-chip">--download</span>
              <span class="ym-chip">--cpu</span>
            </div>
          </div>
          <div class="ym-resource-card">
            <div class="ym-resource-card__eyebrow">Automation</div>
            <div class="ym-resource-card__copy">
              Reuse the same page pipeline in headless mode or through MCP when Yomika
              needs to participate in larger local workflows.
            </div>
            <div class="ym-chip-list">
              <span class="ym-chip">Desktop app</span>
              <span class="ym-chip">Headless mode</span>
              <span class="ym-chip">Local Web UI</span>
              <span class="ym-chip">MCP agent workflows</span>
              <span class="ym-chip">Local integrations</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</div>
