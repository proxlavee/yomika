---
title: Visão geral
social_title: Yomika
description: Yomika é um tradutor de mangá local-first feito em Rust, com OCR, inpainting, suporte a LLMs locais e remotas, Web UI e automação via MCP.
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
          <span>Disponível agora:</span>
          <span class="ym-announce__token">inferência local com llama.cpp</span>
          <span class="ym-announce__copy">
            Rode modelos GGUF localmente com aceleração CUDA, Vulkan ou Metal.
          </span>
        </div>
      </div>

      <div class="ym-hero__copy">
        <h1>Traduza mangá localmente, com privacidade e com um pipeline de produção de verdade.</h1>
        <p class="ym-hero__lede">
          Yomika é um aplicativo desktop em Rust para tradução de mangá. Ele cuida de
          OCR, limpeza, tradução, revisão e exportação no Windows, macOS e Linux.
        </p>
        <div class="ym-hero__model-row">
          <div class="ym-hero__model-label">Modelos locais incluídos</div>
          <div class="ym-chip-list ym-hero__models">
            <span class="ym-chip">sakura</span>
            <span class="ym-chip">vntl-llama3</span>
            <span class="ym-chip">hunyuan</span>
            <span class="ym-chip">lfm2</span>
          </div>
        </div>
        <div class="ym-download-hero">
          <a class="ym-download-button" href="how-to/build-from-source.md">
            Compilar do código-fonte
          </a>
          <div class="ym-download-hero__subtext">
            Gratuito e de código aberto.
          </div>
        </div>
      </div>
    </div>

    <div class="ym-shot">
      <div class="ym-shell">
        <div class="ym-shot__frame">
          <img src="assets/Yomika_Screenshot.png" alt="Editor do Yomika" />
        </div>
      </div>
    </div>
  </section>

  <section class="ym-section">
    <div class="ym-shell">
      <div class="ym-section__header">
        <div class="ym-kicker">Execução sem GUI</div>
        <h2>Rode o Yomika sem a janela do desktop quando você precisar de uma Web UI local ou de um runtime de tradução scriptável.</h2>
        <p>
          O app desktop é a interface principal, mas o mesmo runtime também pode rodar
          headless. Use-o para acesso via navegador, trabalho em lote reprodutível ou
          automação local que ainda dependa do pipeline por página do Yomika.
        </p>
      </div>

      <div class="ym-command-grid">
        <div class="ym-command-card">
          <div class="ym-command-card__title">Modo headless</div>
          <div class="ym-command-card__copy">
            Inicie o Yomika sem a janela do desktop e mantenha o mesmo runtime de
            tradução disponível por meio de uma sessão de navegador em uma porta local fixa.
          </div>
          <pre><code># macOS / Linux
yomika --port 4000 --headless

# Windows
yomika.exe --port 4000 --headless</code></pre>
        </div>
        <div class="ym-command-card">
          <div class="ym-command-card__title">Para que serve o modo headless</div>
          <div class="ym-command-card__copy">
            Use quando você precisar do fluxo do desktop em um formato mais fácil de
            scriptar, agendar ou expor a outras ferramentas locais.
          </div>
          <div class="ym-chip-list">
            <span class="ym-chip">Web UI local</span>
            <span class="ym-chip">Jobs em lote</span>
            <span class="ym-chip">Scripts</span>
            <span class="ym-chip">Host de desktop remoto</span>
          </div>
        </div>
      </div>
    </div>
  </section>

  <section class="ym-section">
    <div class="ym-shell">
      <div class="ym-section__header">
        <div class="ym-kicker">Integração com MCP</div>
        <h2>Deixe agentes operarem o Yomika enquanto os modelos e os dados das páginas permanecem na máquina local.</h2>
        <p>
          O Yomika tem suporte a MCP, então a UI desktop, o modo headless e fluxos de
          trabalho com agentes conversam com o mesmo runtime local de tradução, sem
          se dividirem em stacks separadas.
        </p>
      </div>

      <div class="ym-mcp-grid">
        <div class="ym-mcp-card">
          <h3>Um runtime, vários pontos de entrada</h3>
          <p>
            O mesmo pipeline de páginas alimenta a UI desktop, a Web UI headless e as
            ferramentas MCP, então a automação permanece alinhada com as sessões
            normais de edição.
          </p>
        </div>
        <div class="ym-mcp-card">
          <h3>Tarefas de tradução amigáveis a agentes</h3>
          <p>
            Use agentes para tradução em lote, ciclos de revisão, exportações e
            ferramentas auxiliares que precisem de acesso a OCR, limpeza, tradução e
            saídas em nível de página.
          </p>
        </div>
      </div>
    </div>
  </section>

  <section class="ym-dev">
    <div class="ym-shell">
      <div class="ym-dev__lead">
        <img src="assets/Yomika_Logo.png" alt="Logotipo do Yomika" />
        <div class="ym-kicker">Amigável para desenvolvedores</div>
        <h2>Compile a partir do código-fonte e reutilize o mesmo runtime nas suas próprias ferramentas.</h2>
        <p>
          O Yomika foi pensado para ser prático de compilar e prático de integrar. Use
          Bun e Rust para builds locais, flags estáveis de runtime para deploy e o
          modo headless ou MCP quando você precisar de automação em volta do app.
        </p>
      </div>

      <div class="ym-resource-panel">
        <div class="ym-resource-panel__grid">
          <div class="ym-resource-card">
            <div class="ym-resource-card__eyebrow">Build</div>
            <div class="ym-resource-card__copy">
              Compile o app desktop a partir do código-fonte com o mesmo toolchain
              de Bun e Rust usado pelo projeto.
            </div>
            <pre><code>bun install --frozen-lockfile
bun run build</code></pre>
          </div>
          <div class="ym-resource-card">
            <div class="ym-resource-card__eyebrow">Flags de runtime</div>
            <div class="ym-resource-card__copy">
              O binário do desktop expõe um pequeno conjunto de flags de runtime para
              deploy local e automação, sem introduzir um backend separado.
            </div>
            <div class="ym-chip-list">
              <span class="ym-chip">--headless</span>
              <span class="ym-chip">--port</span>
              <span class="ym-chip">--download</span>
              <span class="ym-chip">--cpu</span>
            </div>
          </div>
          <div class="ym-resource-card">
            <div class="ym-resource-card__eyebrow">Automação</div>
            <div class="ym-resource-card__copy">
              Reutilize o mesmo pipeline de páginas em modo headless ou via MCP quando
              o Yomika precisar participar de fluxos locais maiores.
            </div>
            <div class="ym-chip-list">
              <span class="ym-chip">App desktop</span>
              <span class="ym-chip">Modo headless</span>
              <span class="ym-chip">Web UI local</span>
              <span class="ym-chip">Fluxos com agentes MCP</span>
              <span class="ym-chip">Integrações locais</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</div>
