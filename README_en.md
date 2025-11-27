日本語版は[こちら](./REDME.md)

# Video Capture Program for Basler acA640-750

## Overview

This is a video capture program for testing 1000fps recording using Basler's acA640-750 camera.

Although it may be possible to use the included pylonviewer, I was unsure how to record videos with it, so I created this simple program.

## Operating Environment

Works on both Windows and Linux environments.

I have not tested it on WSL2, as USB recognition can be troublesome.

### Pylon Installation

Download and install the [pylon Software Suite](https://www.baslerweb.com/en/products/software/pylon/) from Basler.

This program has been tested with pylon version 25.09.0.

There may be compatibility issues with USB3, so try changing ports, PCs, or using a different USB board to find a stable environment.

This program uses pylon-cxx with Rust. On Linux, the referenced library names may be outdated, so the following operations are required:

```bash
cd /opt/pylon/lib
sudo ln -s libGenApi_gcc_v3_1_Basler_pylon_v3.so libGenApi_gcc_v3_1_Basler_pylon.so
sudo ln -s libGCBase_gcc_v3_1_Basler_pylon_v3.so libGCBase_gcc_v3_1_Basler_pylon.so
sudo ln -s libLog_gcc_v3_1_Basler_pylon_v3.so libLog_gcc_v3_1_Basler_pylon.so
sudo ln -s libMathParser_gcc_v3_1_Basler_pylon_v3.so libMathParser_gcc_v3_1_Basler_pylon.so
sudo ln -s libXmlParser_gcc_v3_1_Basler_pylon_v3.so libXmlParser_gcc_v3_1_Basler_pylon.so
sudo ln -s libNodeMapData_gcc_v3_1_Basler_pylon_v3.so libNodeMapData_gcc_v3_1_Basler_pylon.so
```

### Rust Installation

Please install [Rust](https://www.rust-lang.org/learn/get-started).

### Build and Run

```bash
cargo run --release
```

This will build and run the program.

To change settings, please modify the source code directly.

Currently, as soon as you run the program, image capture will start automatically and the images will be saved.
