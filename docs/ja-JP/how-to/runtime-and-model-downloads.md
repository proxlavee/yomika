---
title: ランタイムとモデルのダウンロード
---

# ランタイムとモデルのダウンロード

Yomika はローカルファーストですが、初回利用が完全オフラインというわけではありません。ローカルのパイプラインを動かす前に、ネイティブランタイムのファイルやモデルの重みをダウンロードする必要がある場合があります。

これらのダウンロードに失敗する場合は、まずネットワーク経路を確認してください。マシン、ISP、ファイアウォール、プロキシ、または地域制限で到達できないホストからは、Yomika もファイルを取得できません。

## Yomika がダウンロードするもの

Yomika がダウンロードするものは、大きく 3 種類あります。

- `llama.cpp` のバイナリ、対応 NVIDIA 環境の CUDA 支援ファイル、対応 Windows AMD 環境の ZLUDA ファイルなどのネイティブランタイムパッケージ
- 既定のローカルページパイプラインに必要な bootstrap モデルパッケージ
- 後から選ぶ OCR / inpainting エンジンや、ローカル GGUF 翻訳モデルなどのオンデマンドモデルパッケージ

重要な挙動は次の通りです。

- `yomika --download` は bootstrap 対象のランタイムとモデルパッケージを準備して終了する
- ピッカーに表示されるすべてのオプションエンジンやローカル LLM をまとめてダウンロードするわけではない

## ファイルの保存場所

Yomika はランタイムパッケージとモデルキャッシュを、設定された `Data Path` の下に保存します。既定値は、各プラットフォームのローカル app-data ディレクトリ配下の `Yomika` です。

代表的な例:

- Windows: `%LOCALAPPDATA%\Yomika`
- macOS: `~/Library/Application Support/Yomika`
- Linux: `~/.local/share/Yomika`

現在の実装で重要なサブディレクトリは次の通りです。

```text
<Data Path>/
  config.toml
  runtime/
    .downloads/
    cuda/
    llama.cpp/
    zluda/
  models/
    huggingface/
```

意味としては次の通りです。

- `runtime/.downloads` はネイティブランタイムのダウンロードアーカイブをためる共通キャッシュ
- `runtime/*` には Yomika が実際に読み込む展開済みライブラリが入る
- `models/huggingface` は vision モデルとローカル GGUF モデルファイルに使う Hugging Face キャッシュ

すべてのプラットフォームで全部のディレクトリが存在するわけではありません。たとえば `zluda/` は Windows 専用で、対応 AMD 環境でのみ意味があります。

設定済みのパスや HTTP 設定については [設定リファレンス](../reference/settings.md) を参照してください。

## ランタイムのダウンロードはどう動くか

Yomika の起動時、または `yomika --download` 実行時に、現在のプラットフォームと計算ポリシーに応じた bootstrap パッケージを準備します。

大まかな流れは次の通りです。

1. Yomika が現在のデータパス配下にランタイムとモデルのディレクトリを作る。
2. 各 bootstrap パッケージがすでに最新状態か確認する。
3. ネイティブランタイムパッケージが欠けているか古い場合、アーカイブを `runtime/.downloads` にダウンロードする。
4. 必要なファイルをランタイムごとのインストール先ディレクトリに展開し、install marker を書く。
5. ローカルパイプライン開始前にランタイムライブラリを preload する。

現在のソースツリーでは、ネイティブランタイムのダウンロード元はパッケージごとに異なります。

- `llama.cpp` 用の GitHub Releases
- 対応プラットフォーム上の CUDA ランタイム部品用の PyPI メタデータと wheel
- 対応 Windows AMD 環境向け ZLUDA の upstream release asset

つまり、ランタイムダウンロードの失敗が必ず Hugging Face の問題とは限りません。

## モデルのダウンロードはどう動くか

モデルダウンロードの多くは、`models/huggingface` 配下の共通 Hugging Face キャッシュを使います。

大まかな流れは次の通りです。

1. Yomika が特定の `repo/file` の組を要求する。
2. まずローカルの Hugging Face キャッシュを確認する。
3. すでにキャッシュ済みなら、そのまま再利用する。
4. まだ無ければ、そのファイルだけをダウンロードして Hugging Face キャッシュレイアウトに保存する。
5. 以後のロードでは再ダウンロードせず、キャッシュ済みファイルを再利用する。

これは既定の vision モデル群にも、後からオンデマンドでダウンロードするローカル GGUF 翻訳モデルにも当てはまります。

## Hugging Face とは何か

[Hugging Face](https://huggingface.co/) はモデルホスティングのプラットフォームです。Yomika では、多くのモデルファイルの置き場所として使われています。

Hugging Face 自体が Yomika の推論を実行しているわけではありません。Yomika は Hugging Face からモデルファイルをダウンロードしてローカルにキャッシュし、その後はローカルのランタイムスタック上で自分のマシンで実行します。

ネットワーク上で Hugging Face がブロックされている場合、アプリ内のどのボタンを押しても Yomika はそのモデルファイルを取得できません。

## ここでいう「インターネット接続」とは何か

Yomika にとって「ネットにつながっている」とは、「Wi-Fi アイコンがつながっている」「ブラウザで Google が開く」だけでは不十分です。

実際に重要なのは次の点です。

- DNS で必要なホスト名を解決できる
- HTTPS 接続を確立できる
- ファイルダウンロードを開始して最後まで完了できる
- ISP、ファイアウォール、プロキシ、ウイルス対策、または地域制限で対象ホストが遮断されていない

無関係なサイトが開けることは、`huggingface.co`、`github.com`、`pypi.org` に現在のネットワークから到達できる証明にはなりません。

## まず Yomika の外で接続を確認する

これらの確認が Yomika の外で失敗するなら、先にネットワーク経路を直してください。それは Yomika のバグではありません。

### ブラウザでの確認

通常のブラウザで次を開いてください。

- `https://huggingface.co`
- `https://huggingface.co/ogkalu/comic-text-and-bubble-detector`
- `https://github.com`
- `https://pypi.org`

ブラウザで開けないなら、Yomika からも取得できません。

### macOS / Linux での確認

```bash
nslookup huggingface.co
curl -I --max-time 20 https://huggingface.co
curl -I --max-time 20 https://github.com
curl -I --max-time 20 https://pypi.org
curl -L --max-time 20 -o /dev/null -w '%{http_code}\n' \
  https://huggingface.co/ogkalu/comic-text-and-bubble-detector/resolve/main/config.json
```

期待する結果は次の通りです。

- `nslookup` が DNS failure ではなくアドレスを返す
- `curl -I` が `200` のような正常な HTTPS レスポンスを返す
- 直接ファイルの確認コマンドが `200` を出力する

### Windows PowerShell での確認

PowerShell の `curl` エイリアスではなく、`curl.exe` を使ってください。

```powershell
nslookup huggingface.co
curl.exe -I --max-time 20 https://huggingface.co
curl.exe -I --max-time 20 https://github.com
curl.exe -I --max-time 20 https://pypi.org
curl.exe -L --max-time 20 -o NUL -w "%{http_code}\n" `
  https://huggingface.co/ogkalu/comic-text-and-bubble-detector/resolve/main/config.json
```

期待する結果は同じです。DNS が解決でき、HTTPS 応答が返り、直接ファイル確認で `200` が出ることです。

## Hugging Face が地域またはネットワークでブロックされていると分かる兆候

よくある兆候は次の通りです。

- `huggingface.co` だけが timeout、reset、または読み込み完了しないが、無関係な他サイトは開く
- 上の直接ファイル確認で `200` が返らない
- 同じ失敗がブラウザ、`curl`、Yomika のすべてで再現する
- モバイル hotspot など別ネットワークでは動くのに、普段のネットワークでは失敗する
- GitHub や PyPI は使えるが、Hugging Face だけ使えない

ネットワークを変えると解決するなら、原因はネットワーク経路です。

同じマシン上で Yomika の外から見ても Hugging Face が使えないなら、Yomika のバグ報告より先にそちらを解決してください。

## 外部確認が通った後に Yomika で試す

ブラウザと `curl` の確認が通ったら、Yomika 自体をテストしてください。

```bash
# macOS / Linux
yomika --download --debug
yomika --cpu --download --debug

# Windows
yomika.exe --download --debug
yomika.exe --cpu --download --debug
```

両方のコマンドが有用な理由:

- `--download` は通常の bootstrap ダウンロード経路を確認できる
- `--cpu --download` は GPU 優先を外すので、ネットワーク問題と GPU ランタイム準備の問題を切り分けやすい

どちらかが失敗した場合は、正確なエラーテキストを残してください。「download broken」よりはるかに有用です。

## バグ報告前に確認すること

まず次を確認してください。

- 同じマシンのブラウザで Hugging Face、GitHub、PyPI を開けるか
- 上の `curl` 確認が Yomika の外で成功するか
- 別ネットワークに変えると挙動が変わるか
- `--cpu --download` と `--download` で挙動に違いがあるか
- 現在の `Data Path` はどこか
- Yomika が出した正確なエラーテキストは何か

Yomika の外でホストに到達できないなら、先にネットワーク、ファイアウォール、プロキシ、VPN、または ISP 側の問題として対処してください。

外部確認が通っているのに Yomika だけ失敗する場合は、そのときに上の情報を添えて Yomika のバグを報告してください。
