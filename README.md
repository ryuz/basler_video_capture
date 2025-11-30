The English version is available [here](README_en.md).

# Basler acA640-750 用ビデオキャプチャプログラム

## 概要

Basler社の acA640-750 を用いて 1000fps 撮影を試してみたビデオキャプチャプログラムです。

リアルタイムプレビュー表示とキーボード操作による録画機能を備えています。

## 動作環境

Windows もしくは Linux 環境で動作します。 

WSL2 では USB を認識させるのが面倒なので試していません。

### Pylon インストール

Busler社の[pylon Software Suite](https://www.baslerweb.com/ja-jp/software/pylon/)をダウンロードしてインストールしてください。

本プログラムは pylon 25.09.0 で動作確認しています。

稀に USB3 に相性があるようですので、ポートを変えたり、PCを変えたり、USBボードを用意したりして安定する環境を探してください。

Rust で pylon-cxx を利用していますが、Linux版では参照しているライブラリ名が古いようで下記の操作が必要でした。

```bash
cd /opt/pylon/lib
sudo ln -s libGenApi_gcc_v3_1_Basler_pylon_v3.so libGenApi_gcc_v3_1_Basler_pylon.so
sudo ln -s libGCBase_gcc_v3_1_Basler_pylon_v3.so libGCBase_gcc_v3_1_Basler_pylon.so
sudo ln -s libLog_gcc_v3_1_Basler_pylon_v3.so libLog_gcc_v3_1_Basler_pylon.so
sudo ln -s libMathParser_gcc_v3_1_Basler_pylon_v3.so libMathParser_gcc_v3_1_Basler_pylon.so
sudo ln -s libXmlParser_gcc_v3_1_Basler_pylon_v3.so libXmlParser_gcc_v3_1_Basler_pylon.so
sudo ln -s libNodeMapData_gcc_v3_1_Basler_pylon_v3.so libNodeMapData_gcc_v3_1_Basler_pylon.so
```

### Rust インストール

[Rust](https://rust-lang.org/ja/learn/get-started/) をインストールしてください。

### ビルドと実行

```bash
cargo run --release
```

とすればビルドされて実行されます。

## 使い方

### コマンドラインオプション

```bash
cargo run --release -- [オプション]
```

利用可能なオプション:
- `-W, --width <WIDTH>`: 画像幅 (デフォルト: 320)
- `-H, --height <HEIGHT>`: 画像高さ (デフォルト: 320)
- `-r, --rec-frames <REC_FRAMES>`: 録画フレーム数 (デフォルト: 1000)
- `-e, --exposure <EXPOSURE>`: 露光時間 (デフォルト: 800.0)
- `-f, --format <FORMAT>`: ピクセルフォーマット (デフォルト: Mono10)

### 操作方法

プログラムを起動するとプレビューウィンドウが表示され、リアルタイムでカメラ映像が表示されます。

- **R キー**: 録画開始。指定したフレーム数分の画像を連続撮影し、タイムスタンプ付きのディレクトリに保存します
- **ESC キー**: プログラムを終了

### 録画データ

録画された画像は `rec/YYYYMMDD_HHMMSS/` ディレクトリに PGM (P2 ASCII) 形式で保存されます。

ファイル名は `image_0000.pgm`, `image_0001.pgm`, ... となります。

PGM 形式で保存されますので、必要に応じて動画編集ソフト等で動画に変換してください。

