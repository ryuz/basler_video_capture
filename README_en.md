日本語版は[こちら](./REDME.md)

# Video Capture Program for Basler acA640-750

## Overview

This is a video capture program for testing 1000fps recording using Basler's acA640-750 camera.

It features real-time preview display and keyboard-controlled recording functionality.

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

## Usage

### Command-line Options

```bash
cargo run --release -- [OPTIONS]
```

Available options:
- `-W, --width <WIDTH>`: Image width (default: 320)
- `-H, --height <HEIGHT>`: Image height (default: 320)
- `-r, --rec-frames <REC_FRAMES>`: Number of frames to record (default: 1000)
- `-e, --exposure <EXPOSURE>`: Exposure time (default: 800.0)
- `-f, --format <FORMAT>`: Pixel format (default: Mono10)

### Controls

When you run the program, a preview window will open displaying the real-time camera feed.

- **R key**: Start recording. Captures the specified number of frames continuously and saves them to a timestamped directory
- **ESC key**: Exit the program

### Recorded Data

Recorded images are saved in PGM (P2 ASCII) format in the `rec/YYYYMMDD_HHMMSS/` directory.

File names follow the pattern `image_0000.pgm`, `image_0001.pgm`, etc.

You can convert the PGM images to video format using video editing software if needed.
