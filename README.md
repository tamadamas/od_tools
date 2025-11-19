# Log generator for [OpenDominion Game](https://github.com/OpenDominion/OpenDominion)

Available for Linux, Windows, and macOS

Converted from the script included in sim file from [Yami-10/OD-Simulator](https://github.com/Yami-10/OD-Simulator)

# The idea

I work on Linux so don't have Excel and has no ability to generate `log.txt` from sim file.

This tool can generate `log.txt` from Excel file and should work in any Operation System.

The code contains only final script generation logic. All other things left the same in Excel file.

Remember to open Excel sim before generation. ALl formulas should run and update their values with Excel.

# Installation

### Download from [GitHub Release](https://github.com/tamadamas/od_tools/releases)

### Install prebuilt binaries via shell script

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/tamadamas/od_tools/releases/download/v2.0.0/od_tools-installer.sh | sh
```

### Install prebuilt binaries via Homebrew

```sh
brew install tamadamas/tap/od_tools
```

### Install prebuilt binaries via [Cargo binstall](https://github.com/cargo-bins/cargo-binstall)

```sh
cargo binstall od_tools
```

### Build From Source with [Rust Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

```
cargo install cargo-dist --locked
```

# Usage

Next will work for Linux and Mac

```
sim generate_log generate-log --sim sim.xlsx --result sim.txt
```

Get file from [Yami-10/OD-Simulator](https://github.com/Yami-10/OD-Simulator)

# Bug reports

If you see any issues or want an improvement, feel free to create an issue and describe the problem.

Write me in Discord @tomas_tamadamas
