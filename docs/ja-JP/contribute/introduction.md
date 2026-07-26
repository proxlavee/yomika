---
title: はじめに
---

# Yomika へのコントリビュート

Yomika に興味を持っていただきありがとうございます。Yomika はローカルファーストで動く、Rust 製の ML パワードな漫画翻訳ツールです。あなたの協力を歓迎します。

## クイックスタート

一番早いのは [good first issues](https://github.com/proxlavee/yomika/contribute) から選ぶ方法です。新しいコントリビューター向けに厳選したタスクを置いています。

相談したいときは [GitHub Discussions](https://github.com/proxlavee/yomika/discussions) または関連する Issue を利用してください。

## コントリビュートの方法

どんな形のコントリビューションも歓迎します。

### バグ報告

- 検出・OCR・インペイント・翻訳パイプラインの不具合
- クラッシュ、リグレッション、パフォーマンスの低下
- レンダリング、PSD エクスポート、プロバイダ連携のエッジケース

### 機能開発

- OCR、検出、インペイント、LLM バックエンドの追加
- テキストレンダラー、HTTP API、MCP サーバの改善
- UI のパネル、ショートカット、ワークフローの拡張

### ドキュメント

- Getting Started や How-To の改善
- 例、スクリーンショット、チュートリアルの追加
- 他言語への翻訳

### テスト

- ワークスペース各クレートの Rust ユニットテスト
- `ui/tests/` の Vitest テストと `tests/integration-tests/` の Rust 統合テストの拡充
- OCR / 検出用の実在漫画ページの提供

### インフラ

- ビルドと CI の改善
- モデルダウンロード、ランタイムキャッシュ、アクセラレーションの最適化
- Windows、macOS、Linux のパッケージングを健全に保つ

## コードベースの構造

Yomika は Rust ワークスペースに Tauri シェルと Next.js UI を組み合わせた構成です。

- **`crates/yomika/`** — Tauri のデスクトップシェル
- **`crates/yomika-app/`** — アプリ側バックエンドとパイプラインのオーケストレーション
- **`crates/yomika-core/`** — 共有型、イベント、ユーティリティ
- **`crates/yomika-ml/`** — 検出、OCR、インペイント、フォント解析
- **`crates/yomika-llm/`** — llama.cpp バインディングと LLM プロバイダ
- **`crates/yomika-renderer/`** — テキストシェーピングとレンダリング
- **`crates/yomika-psd/`** — レイヤー付き PSD エクスポート
- **`crates/yomika-rpc/`** — HTTP API と MCP サーバ
- **`crates/yomika-runtime/`** — ランタイムとモデルダウンロードの管理
- **`ui/`** — Next.js 製 Web UI
- **`tests/integration-tests/`** — Rust の HTTP・アプリ統合テスト
- **`ui/tests/`** — Vitest による UI・フロントエンド単体テスト
- **`docs/`** — ドキュメントサイト (English, 日本語, 简体中文, Português)

## はじめてのコントリビューション

1. **Issue を眺める** — [`good first issue`](https://github.com/proxlavee/yomika/labels/good%20first%20issue) から始めます。
2. **遠慮なく質問する** — 関連する Issue または GitHub Discussions を利用してください。
3. **小さく始める** — ドキュメントの修正や絞った範囲のバグ修正がいちばん通しやすいです。
4. **コードを読む** — 編集しているファイルの既存パターンに合わせます。

## コミュニティ

### コミュニケーション

- **[GitHub Discussions](https://github.com/proxlavee/yomika/discussions)** — 設計に関する議論や質問
- **[GitHub Issues](https://github.com/proxlavee/yomika/issues)** — バグ報告と機能要望

### AI 利用ポリシー

Yomika へのコントリビュートに AI ツール (ChatGPT、Claude、Copilot などの LLM) を使う場合:

- **AI の利用を明示してください** — メンテナーの負担を減らすためです
- **あなたが責任を負います** — 自分が提出した Issue や PR の中身はすべて自分の責任です
- **品質の低い未レビューの AI 生成物はその場でクローズします**
- **低品質または未確認の投稿はクローズされる場合があります。** 投稿するすべての変更を理解し、検証する責任はコントリビューターにあります。

開発補助として AI を使うのは歓迎しますが、提出前にコントリビューター本人が十分にレビューしてテストしてください。AI が生成したコードは理解し、検証し、Yomika の水準に合わせて調整したうえで提出してください。

## 次のステップ

始める準備ができたら:

- **ローカル環境をセットアップする** — [Getting Started](development.md)
- **Issue を選ぶ** — [good first issues](https://github.com/proxlavee/yomika/contribute)
- **アイデアを相談する** — [GitHub Discussions](https://github.com/proxlavee/yomika/discussions) を開始する
- **パイプラインを学ぶ** — [Yomika の仕組み](../explanation/how-yomika-works.md) と [テクニカル詳細](../explanation/technical-deep-dive.md)
