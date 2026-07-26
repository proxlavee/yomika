---
title: 概要
social_title: Yomika
description: Yomika は OCR、インペインティング、翻訳、組版、書き出し、自動化を一つにまとめたローカルファーストのマンガ翻訳環境です。
hide:
  - navigation
  - toc
---

<div class="ym-home">
  <div class="ym-shell">
    <section class="ym-hero" aria-labelledby="ym-hero-title">
      <div class="ym-hero__grid">
        <div class="ym-hero__copy">
          <div class="ym-kicker">PAGE 01 · ローカルファーストのマンガ制作</div>
          <h1 id="ym-hero-title">原稿から、<span>仕上がった写植</span>まで。</h1>
          <p class="ym-hero__lede">
            Yomika は検出、OCR、インペインティング、翻訳、校正、組版、書き出しを、
            ページ構造を理解する一つのワークスペースにまとめます。内蔵パイプラインはローカルで動かし、
            必要なプロジェクトだけリモートプロバイダーを選べます。
          </p>
          <div class="ym-actions">
            <a class="ym-button ym-button--primary" href="how-to/build-from-source.md">Yomika をビルド</a>
            <a class="ym-button ym-button--secondary" href="tutorials/translate-your-first-page.md">最初のページを翻訳</a>
          </div>
          <ul class="ym-facts" aria-label="Yomika の概要">
            <li>Windows・Linux・Apple Silicon</li>
            <li>GPL-3.0</li>
            <li>デスクトップ・Headless・MCP</li>
          </ul>
        </div>

        <div class="ym-hero__visual">
          <div class="ym-panel-label">EDITOR / PAGE VIEW</div>
          <div class="ym-screen">
            <img src="assets/Yomika_Screenshot.png" alt="翻訳したマンガページを編集中の Yomika" />
          </div>
          <div class="ym-callout ym-callout--top">
            <strong>標準でローカル</strong>
            <span>Vision とダウンロード済み LLM は端末上で動作。</span>
          </div>
          <div class="ym-callout ym-callout--bottom">
            <strong>編集できる受け渡し</strong>
            <span>完成画像またはレイヤー付き PSD を書き出し。</span>
          </div>
        </div>
      </div>
    </section>

    <section class="ym-workflow" aria-labelledby="ym-workflow-title">
      <div class="ym-section-heading">
        <div>
          <div class="ym-kicker">1 ページ、途切れない工程</div>
          <h2 id="ym-workflow-title">つながったスキャンレーション環境。</h2>
        </div>
        <p>
          別々のツールへ素材を移さず、各工程の結果を次へ渡せます。
          検出ブロック、OCR テキスト、最後の写植はいつでも見直して修正できます。
        </p>
      </div>
      <ol class="ym-steps">
        <li class="ym-step">
          <span class="ym-step__number">01</span>
          <strong>検出</strong>
          <p>テキスト領域、吹き出し、ページ構造を見つけます。</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">02</span>
          <strong>OCR</strong>
          <p>台詞やキャプションを校正可能な Unicode テキストへ変換します。</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">03</span>
          <strong>消去</strong>
          <p>絵を保ちながら原文の文字をインペインティングします。</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">04</span>
          <strong>翻訳</strong>
          <p>ローカル GGUF モデルまたは任意の外部プロバイダーを使います。</p>
        </li>
        <li class="ym-step">
          <span class="ym-step__number">05</span>
          <strong>組版</strong>
          <p>校正、レンダリング、完成画像やレイヤー付き PSD の書き出し。</p>
        </li>
      </ol>
    </section>

    <section class="ym-section" aria-labelledby="ym-modes-title">
      <div class="ym-section-heading">
        <div>
          <div class="ym-kicker">作業環境を選ぶ</div>
          <h2 id="ym-modes-title">一つのランタイム、三つの使い方。</h2>
        </div>
        <p>
          デスクトップエディター、Headless Web UI、HTTP API、MCP ツールは、
          同じプロジェクト状態とページパイプラインを共有します。
        </p>
      </div>
      <div class="ym-mode-grid">
        <a class="ym-mode-card ym-mode-card--desktop" href="tutorials/translate-your-first-page.md">
          <span class="ym-mode-tag">デスクトップエディター</span>
          <h3>吹き出し、マスク、すべてのテキストを手元で調整。</h3>
          <p>
            ページ一式を読み込み、工程ごとに実行し、マスクを修復し、写植を整え、
            編集可能な形式のまま書き出せます。
          </p>
          <ul class="ym-pills">
            <li>複数ページ</li>
            <li>縦書き CJK</li>
            <li>RTL レイアウト</li>
            <li>レイヤー付き PSD</li>
          </ul>
          <span class="ym-mode-card__arrow" aria-hidden="true">↗</span>
        </a>
        <a class="ym-mode-card" href="how-to/run-gui-headless-and-mcp.md">
          <span class="ym-mode-tag">Headless</span>
          <h3>同じワークスペースをブラウザーで。</h3>
          <p>ウィンドウを開かず、スクリプト、バッチ処理、固定ローカルサーバーとして実行します。</p>
          <span class="ym-mode-card__arrow" aria-hidden="true">↗</span>
        </a>
        <a class="ym-mode-card" href="how-to/configure-mcp-clients.md">
          <span class="ym-mode-tag">MCP + HTTP API</span>
          <h3>実際のプロジェクト状態をエージェントに接続。</h3>
          <p>通常の編集とエージェント操作を揃えたまま、プロジェクト作業を自動化します。</p>
          <span class="ym-mode-card__arrow" aria-hidden="true">↗</span>
        </a>
      </div>
    </section>

    <section class="ym-privacy" aria-labelledby="ym-privacy-title">
      <div class="ym-privacy__copy">
        <div class="ym-kicker">ローカルファースト、ローカル限定ではない</div>
        <h2 id="ym-privacy-title">ページは手元に。クラウドは選んだときだけ。</h2>
        <p>
          Vision スタックとダウンロード済み翻訳モデルは端末上で実行できます。
          リモート LLM、機械翻訳、Codex の各ワークフローは、隠れた依存ではなく明示的な選択肢です。
        </p>
      </div>
      <ul class="ym-privacy__list">
        <li>
          <span class="ym-privacy__mark">A</span>
          <span><strong>ローカルパイプライン</strong>検出、OCR、消去、ローカル翻訳を端末上に保てます。</span>
        </li>
        <li>
          <span class="ym-privacy__mark">B</span>
          <span><strong>プロバイダーを管理</strong>テキストや画像を外部サービスへ送るタイミングを選べます。</span>
        </li>
        <li>
          <span class="ym-privacy__mark">C</span>
          <span><strong>実用的な出力</strong>完成画像、編集可能な PSD レイヤー、プロジェクトアーカイブを保存できます。</span>
        </li>
      </ul>
    </section>

    <section class="ym-install" aria-labelledby="ym-install-title">
      <div class="ym-install__grid">
        <div class="ym-install__copy">
          <div class="ym-kicker">ソースビルド</div>
          <h2 id="ym-install-title">最初のページを始めますか？</h2>
          <p>
            現在、ビルド済みリリースは公開されていません。正しい Tauri とプラットフォーム機能を使うため、
            リポジトリの Bun ラッパーでビルドしてください。
          </p>
        </div>
        <div>
          <pre><code>git clone https://github.com/proxlavee/yomika.git
cd yomika
bun install --frozen-lockfile
bun run build</code></pre>
          <div class="ym-install__links">
            <a class="ym-text-link" href="how-to/build-from-source.md">必要条件とプラットフォーム別注意点</a>
            <a class="ym-text-link" href="how-to/runtime-and-model-downloads.md">初回ダウンロードについて</a>
            <a class="ym-text-link" href="https://github.com/proxlavee/yomika">GitHub でソースを見る</a>
          </div>
        </div>
      </div>
    </section>
  </div>
</div>
