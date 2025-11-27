The English version is available [here](README_en.md).

# Basler acA640-750 用ビデオキャプチャプログラム

## 概要

Basler社の acA640-750 を用いて 1000fps 撮影を試してみたビデオキャプチャプログラムです。

付属の pylonviewer でもできるのかもしれませんが、動画撮影やり方がよくわからなかったので簡易的に作成しました。

## 動作環境

Windows もしくは Linux 環境で動作します。 

WSL2 では USB を認識させるのが面倒なので試していません。

### Pylon インストール

Busler社の[pylon Software Suite](https://www.baslerweb.com/ja-jp/software/pylon/)をダウンロードしてインストールしてください。

本プログラムは pylon 25.09.0 で動作確認しています。

USB3 に相性があるようですので、ポートを変えたり、PCを変えたり、USBボードを用意したりして安定する環境を探してください。

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

設定変更はソースコードを書き換えてください。

現時点では実行と同時に撮影が始まり、画像が保存されるだけです。

PGM 形式で保存されますので、必要に応じて動画編集ソフト等で動画に変換してください。

そのうち時間が出来たらプレビュー機能なども追加したいと思います。

