# Basler acA640-750 用ビデオキャプチャプログラム

## 概要

Basler社の acA640-750 を用いて 1000fps 撮影を試してみたビデオキャプチャプログラムです。

付属の pylonviewer でもできるのかもしれませんが、動画撮影やり方がよくわからなかったので簡易的に作成しました。

## 動作環境


### Pylon インストール

Windows もしくは Linux 環境で動作します。 pylon 25.09.0 をインストールしてください。

WSL2 では USB を認識させるのが面倒なので試していません。

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

