---
title: Visão geral
social_title: Yomika
description: Yomika é um espaço de trabalho local-first para OCR, inpainting, tradução, diagramação, exportação e automação de mangás.
hide:
  - navigation
  - toc
---

<div class="ym-home">
  <div class="ym-shell">
    <section class="ym-hero" aria-labelledby="ym-hero-title">
      <div class="ym-hero__grid">
        <div class="ym-hero__copy">
          <div class="ym-kicker">PAGE 01 · Produção de mangá local-first</div>
          <h1 id="ym-hero-title">Da página original à <span>diagramação final.</span></h1>
          <p class="ym-hero__lede">
            O Yomika reúne detecção, OCR, inpainting, tradução, revisão,
            diagramação e exportação em um único espaço que entende cada página.
            Rode o pipeline integrado localmente e escolha um provedor remoto
            somente quando o projeto precisar.
          </p>
          <div class="ym-actions">
            <a class="ym-button ym-button--primary" href="https://github.com/proxlavee/yomika/releases/latest">Baixar para Windows</a>
            <a class="ym-button ym-button--secondary" href="tutorials/translate-your-first-page.md">Traduzir a primeira página</a>
          </div>
          <ul class="ym-facts" aria-label="Resumo do Yomika">
            <li>Versão portátil para Windows</li>
            <li>Builds a partir do código-fonte para Linux e macOS</li>
            <li>GPL-3.0</li>
          </ul>
        </div>

        <div class="ym-hero__visual">
          <div class="ym-panel-label">EDITOR / PAGE VIEW</div>
          <div class="ym-screen">
            <img src="assets/Yomika_Screenshot.png" alt="Yomika editando uma página de mangá traduzida" />
          </div>
          <div class="ym-callout ym-callout--top">
            <strong>Local por padrão</strong>
            <span>Visão e LLMs baixados rodam no dispositivo.</span>
          </div>
          <div class="ym-callout ym-callout--bottom">
            <strong>Entrega editável</strong>
            <span>Exporte páginas prontas ou arquivos PSD em camadas.</span>
          </div>
        </div>
      </div>
    </section>

    <section class="ym-workflow" aria-labelledby="ym-workflow-title">
      <div class="ym-section-heading">
        <div>
          <div class="ym-kicker">Uma página, um fluxo contínuo</div>
          <h2 id="ym-workflow-title">A bancada de scanlation, conectada.</h2>
        </div>
        <p>
          Cada etapa alimenta a próxima sem mover arquivos entre ferramentas
          desconectadas. Revise blocos detectados, corrija o texto e refine a
          diagramação final a qualquer momento.
        </p>
      </div>
      <ol class="ym-steps">
        <li class="ym-step">
          <span class="ym-step__number">01</span>
          <strong>Detectar</strong>
          <p>Encontre regiões de texto, balões de fala e a estrutura da página.</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">02</span>
          <strong>OCR</strong>
          <p>Converta diálogos e legendas em texto Unicode revisável.</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">03</span>
          <strong>Inpainting</strong>
          <p>Remova o texto original preservando a arte da página.</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">04</span>
          <strong>Traduzir</strong>
          <p>Use um modelo GGUF local ou um provedor remoto opcional.</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">05</span>
          <strong>Diagramar</strong>
          <p>Revise, renderize e exporte a página final ou um PSD em camadas.</p>
        </li>
      </ol>
    </section>

    <section class="ym-section" aria-labelledby="ym-modes-title">
      <div class="ym-section-heading">
        <div>
          <div class="ym-kicker">Escolha sua bancada</div>
          <h2 id="ym-modes-title">Um runtime, três formas de trabalhar.</h2>
        </div>
        <p>
          O editor desktop, a Web UI headless, a API HTTP e as ferramentas MCP
          usam o mesmo estado de projeto e o mesmo pipeline de páginas.
        </p>
      </div>
      <div class="ym-mode-grid">
        <a class="ym-mode-card ym-mode-card--desktop" href="tutorials/translate-your-first-page.md">
          <span class="ym-mode-tag">Editor desktop</span>
          <h3>Ajuste cada balão, máscara e linha de texto.</h3>
          <p>
            Importe conjuntos de páginas, rode etapas separadamente, repare
            máscaras, refine a diagramação e preserve a edição na exportação.
          </p>
          <ul class="ym-pills">
            <li>Projetos em lote</li>
            <li>CJK vertical</li>
            <li>Layout RTL</li>
            <li>PSD em camadas</li>
          </ul>
          <span class="ym-mode-card__arrow" aria-hidden="true">↗</span>
        </a>
        <a class="ym-mode-card" href="how-to/run-gui-headless-and-mcp.md">
          <span class="ym-mode-tag">Headless</span>
          <h3>Abra o mesmo espaço de trabalho no navegador.</h3>
          <p>Rode sem janela desktop para scripts, tarefas em lote ou um servidor local fixo.</p>
          <span class="ym-mode-card__arrow" aria-hidden="true">↗</span>
        </a>
        <a class="ym-mode-card" href="how-to/configure-mcp-clients.md">
          <span class="ym-mode-tag">MCP + API HTTP</span>
          <h3>Conecte agentes ao estado real do projeto.</h3>
          <p>Automatize tarefas mantendo a edição normal e as ações do agente alinhadas.</p>
          <span class="ym-mode-card__arrow" aria-hidden="true">↗</span>
        </a>
      </div>
    </section>

    <section class="ym-privacy" aria-labelledby="ym-privacy-title">
      <div class="ym-privacy__copy">
        <div class="ym-kicker">Local-first, não apenas local</div>
        <h2 id="ym-privacy-title">Mantenha a página perto. Use a nuvem por escolha.</h2>
        <p>
          O Yomika pode rodar o stack visual e os modelos de tradução baixados
          na sua máquina. LLMs remotos, tradução automática e fluxos com Codex
          são opções explícitas, não dependências ocultas.
        </p>
      </div>
      <ul class="ym-privacy__list">
        <li>
          <span class="ym-privacy__mark">A</span>
          <span><strong>Pipeline local</strong>Detecção, OCR, limpeza e tradução local podem ficar no dispositivo.</span>
        </li>
        <li>
          <span class="ym-privacy__mark">B</span>
          <span><strong>Controle de provedores</strong>Você escolhe quando texto ou imagens vão para um serviço configurado.</span>
        </li>
        <li>
          <span class="ym-privacy__mark">C</span>
          <span><strong>Saída prática</strong>Salve imagens prontas, camadas PSD editáveis ou arquivos de projeto.</span>
        </li>
      </ul>
    </section>

    <section class="ym-install" aria-labelledby="ym-install-title">
      <div class="ym-install__grid">
        <div class="ym-install__copy">
          <div class="ym-kicker">Obtenha o Yomika</div>
          <h2 id="ym-install-title">Vamos começar a primeira página?</h2>
          <p>
            Escolha o EXE ou ZIP portátil para Windows, sem instalador. Use o
            wrapper Bun do repositório no Linux, macOS ou para uma build personalizada.
          </p>
        </div>
        <div>
          <pre><code>git clone https://github.com/proxlavee/yomika.git
cd yomika
bun install --frozen-lockfile
bun run build</code></pre>
          <div class="ym-install__links">
            <a class="ym-text-link" href="https://github.com/proxlavee/yomika/releases/latest">Baixar o EXE ou ZIP para Windows</a>
            <a class="ym-text-link" href="how-to/build-from-source.md">Ver pré-requisitos e notas da plataforma</a>
            <a class="ym-text-link" href="how-to/runtime-and-model-downloads.md">Entender os downloads da primeira execução</a>
            <a class="ym-text-link" href="https://github.com/proxlavee/yomika">Ver o código no GitHub</a>
          </div>
        </div>
      </div>
    </section>
  </div>
</div>
