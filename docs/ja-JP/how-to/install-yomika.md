---
title: Yomika をインストールする
---

# Yomika をインストールする

## Windows 版をダウンロードする

[Yomika Releases ページ](https://github.com/proxlavee/yomika/releases/latest) から、最新の Windows 向けポータブル `.exe` または `.zip` をダウンロードしてください。ZIP には同じ実行ファイルが含まれています。任意のフォルダーに展開し、`Yomika-<version>-windows-x64.exe` を実行します。インストーラーは使用しません。開発用または独自のビルドについては、[ソースからビルドする](build-from-source.md) を参照してください。

## ローカルに保存されるもの

Yomika は local-first のアプリです。ポータブル実行ファイルとは別に、初回起動時にはユーザーごとのローカルデータディレクトリが作成され、次のものが保存されます。

- llama.cpp や GPU バックエンドで使うランタイムライブラリ
- ダウンロードされた vision / OCR モデル
- あとから選択したローカル翻訳モデル

Yomika はアプリ本体のデータを `Yomika` の app-data ルート以下に保持し、モデルの重みはアプリバイナリ本体とは別に管理します。

## 初回起動時に起きること

初回起動時、Yomika は次を行うことがあります。

- ローカル推論スタックに必要なランタイムライブラリを展開またはダウンロードする
- 検出、segmentation、OCR、inpainting、フォント推定で使う既定の vision モデル群をダウンロードする
- ローカル翻訳 LLM は、モデルピッカーで **Download** を選択するまでダウンロードしない

これは正常な挙動であり、回線速度やハードウェアによっては時間がかかります。
モデルのダウンロード中は進捗を確認してキャンセルでき、完了すると通知が表示されます。モデルライブラリの場所の変更、ダウンロード済みモデルの削除、再ダウンロードは **Settings > Runtime** で行えます。

これらのランタイム依存物を先に取得したい場合は、`--download` 付きで一度 Yomika を実行してください。この経路ではランタイムパッケージと既定の vision スタックを初期化したあと、GUI を開かずに終了します。

## アプリの更新

Yomika は起動時に最新の GitHub リリースを確認します。**Settings > About** から手動で確認することもできます。新しいバージョンがある場合は Releases ページを開く通知が表示されますが、更新を自動でダウンロードまたはインストールすることはありません。

## GPU アクセラレーションに関する注意

Yomika は次をサポートしています。

- 対応する NVIDIA GPU 上の CUDA
- Apple Silicon Mac 上の Metal
- Windows / Linux 上での OCR と LLM 推論向け Vulkan
- 全プラットフォームでの CPU フォールバック

実際には次の点が重要です。

- 検出と inpainting は CUDA または Metal の恩恵が大きい
- Vulkan は主に OCR とローカル LLM 推論のための代替 GPU 経路
- NVIDIA ドライバが CUDA 13.0 以降に対応していると確認できない場合、Yomika は CPU にフォールバックする

CUDA 対応環境では、必要なランタイム部品を手作業でライブラリパス設定しなくてもよいように、Yomika が自前で同梱・初期化します。

!!! note

    NVIDIA ドライバは最新に保ってください。Yomika は vision GPU アクセラレーション用に CUDA 13.0 以降対応のドライバを必要とし、Windows のローカル LLM CUDA 経路では CUDA 13.1+ を要求します。ドライバが古い場合は CPU にフォールバックします。

## インストール後に決めること

Yomika が正常に起動したら、次に考えることはたいてい以下です。

- デスクトップ GUI を使うか、headless モードを使うか
- ローカル翻訳モデルを使うか、リモートプロバイダを使うか
- rendered export にするか、レイヤー付き PSD export にするか

続けて読むページ:

- [GUI / Headless / MCP モードを使う](run-gui-headless-and-mcp.md)
- [モデルとプロバイダ](../explanation/models-and-providers.md)
- [ページを書き出し、プロジェクトを管理する](export-and-manage-projects.md)
- [トラブルシューティング](troubleshooting.md)

## サポートが必要な場合

[GitHub Issues](https://github.com/proxlavee/yomika/issues) で既存の報告を検索するか、新しい報告を作成してください。
