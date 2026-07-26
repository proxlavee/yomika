---
title: Overview
social_title: Yomika
description: Yomika is a local-first manga translation workspace for OCR, inpainting, translation, typesetting, export, and automation.
hide:
  - navigation
  - toc
---

<div class="ym-home">
  <div class="ym-shell">
    <section class="ym-hero" aria-labelledby="ym-hero-title">
      <div class="ym-hero__grid">
        <div class="ym-hero__copy">
          <div class="ym-kicker">Page 01 · Local-first manga production</div>
          <h1 id="ym-hero-title">Turn a raw manga page into <span>finished lettering.</span></h1>
          <p class="ym-hero__lede">
            Yomika brings detection, OCR, inpainting, translation, review,
            typesetting, and export into one page-aware workspace. Run the
            built-in pipeline locally, then opt into a hosted provider only
            when your project needs one.
          </p>
          <div class="ym-actions">
            <a class="ym-button ym-button--primary" href="how-to/build-from-source.md">Build Yomika</a>
            <a class="ym-button ym-button--secondary" href="tutorials/translate-your-first-page.md">Translate your first page</a>
          </div>
          <ul class="ym-facts" aria-label="Yomika facts">
            <li>Windows, Linux, Apple Silicon</li>
            <li>GPL-3.0</li>
            <li>Desktop, headless, MCP</li>
          </ul>
        </div>

        <div class="ym-hero__visual">
          <div class="ym-panel-label">EDITOR / PAGE VIEW</div>
          <div class="ym-screen">
            <img src="assets/Yomika_Screenshot_en.png" alt="Yomika editing a translated manga page" />
          </div>
          <div class="ym-callout ym-callout--top">
            <strong>Local by default</strong>
            <span>Vision and downloaded LLMs stay on-device.</span>
          </div>
          <div class="ym-callout ym-callout--bottom">
            <strong>Editable handoff</strong>
            <span>Export rendered pages or layered PSD files.</span>
          </div>
        </div>
      </div>
    </section>

    <section class="ym-workflow" aria-labelledby="ym-workflow-title">
      <div class="ym-section-heading">
        <div>
          <div class="ym-kicker">One page, one continuous flow</div>
          <h2 id="ym-workflow-title">The scanlation desk, connected.</h2>
        </div>
        <p>
          Each stage feeds the next without moving assets between unrelated
          tools. Review the detected blocks, correct the text, and refine the
          final lettering at any point.
        </p>
      </div>
      <ol class="ym-steps">
        <li class="ym-step">
          <span class="ym-step__number">01</span>
          <strong>Detect</strong>
          <p>Find text regions, speech bubbles, and page structure.</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">02</span>
          <strong>OCR</strong>
          <p>Read dialogue and captions into reviewable Unicode text.</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">03</span>
          <strong>Inpaint</strong>
          <p>Remove the source lettering while preserving the artwork.</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">04</span>
          <strong>Translate</strong>
          <p>Use a local GGUF model or an optional remote provider.</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">05</span>
          <strong>Typeset</strong>
          <p>Review, render, and export the finished page or layered PSD.</p>
        </li>
      </ol>
    </section>

    <section class="ym-section" aria-labelledby="ym-modes-title">
      <div class="ym-section-heading">
        <div>
          <div class="ym-kicker">Choose your desk</div>
          <h2 id="ym-modes-title">One runtime, three ways to work.</h2>
        </div>
        <p>
          The desktop editor, headless Web UI, HTTP API, and MCP tools all use
          the same project state and page pipeline.
        </p>
      </div>
      <div class="ym-mode-grid">
        <a class="ym-mode-card ym-mode-card--desktop" href="tutorials/translate-your-first-page.md">
          <span class="ym-mode-tag">Desktop editor</span>
          <h3>Stay close to every bubble, mask, and line of text.</h3>
          <p>
            Import page sets, run individual stages, repair masks, tune
            lettering, and keep the result editable through export.
          </p>
          <ul class="ym-pills">
            <li>Batch projects</li>
            <li>Vertical CJK</li>
            <li>RTL layout</li>
            <li>Layered PSD</li>
          </ul>
          <span class="ym-mode-card__arrow" aria-hidden="true">↗</span>
        </a>
        <a class="ym-mode-card" href="how-to/run-gui-headless-and-mcp.md">
          <span class="ym-mode-tag">Headless</span>
          <h3>Open the same workspace in a browser.</h3>
          <p>Run without a desktop window for scripts, batch jobs, or a fixed local server.</p>
          <span class="ym-mode-card__arrow" aria-hidden="true">↗</span>
        </a>
        <a class="ym-mode-card" href="how-to/configure-mcp-clients.md">
          <span class="ym-mode-tag">MCP + HTTP API</span>
          <h3>Let agents work with the real project state.</h3>
          <p>Automate project tasks while keeping normal editing and agent actions aligned.</p>
          <span class="ym-mode-card__arrow" aria-hidden="true">↗</span>
        </a>
      </div>
    </section>

    <section class="ym-privacy" aria-labelledby="ym-privacy-title">
      <div class="ym-privacy__copy">
        <div class="ym-kicker">Local-first, not local-only</div>
        <h2 id="ym-privacy-title">Keep the page close. Reach the cloud by choice.</h2>
        <p>
          Yomika can run its vision stack and downloaded translation models on
          your machine. Remote LLM, machine-translation, and Codex workflows
          are explicit options rather than hidden dependencies.
        </p>
      </div>
      <ul class="ym-privacy__list">
        <li>
          <span class="ym-privacy__mark">A</span>
          <span><strong>Local pipeline</strong>Detection, OCR, cleanup, and local translation can stay on-device.</span>
        </li>
        <li>
          <span class="ym-privacy__mark">B</span>
          <span><strong>Provider control</strong>Choose when text or images are sent to a configured service.</span>
        </li>
        <li>
          <span class="ym-privacy__mark">C</span>
          <span><strong>Practical output</strong>Keep flattened images, editable PSD layers, or project archives.</span>
        </li>
      </ul>
    </section>

    <section class="ym-install" aria-labelledby="ym-install-title">
      <div class="ym-install__grid">
        <div class="ym-install__copy">
          <div class="ym-kicker">Install Yomika</div>
          <h2 id="ym-install-title">Ready to turn the first page?</h2>
          <p>
            Download the latest desktop package, or use the repository's Bun
            wrapper when you need a custom source build.
          </p>
        </div>
        <div>
          <pre><code>git clone https://github.com/proxlavee/yomika.git
cd yomika
bun install --frozen-lockfile
bun run build</code></pre>
          <div class="ym-install__links">
            <a class="ym-text-link" href="https://github.com/proxlavee/yomika/releases/latest">Download the latest release</a>
            <a class="ym-text-link" href="how-to/build-from-source.md">Read prerequisites and platform notes</a>
            <a class="ym-text-link" href="how-to/runtime-and-model-downloads.md">Understand first-run downloads</a>
            <a class="ym-text-link" href="https://github.com/proxlavee/yomika">View the source on GitHub</a>
          </div>
        </div>
      </div>
    </section>
  </div>
</div>
