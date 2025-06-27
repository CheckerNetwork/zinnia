<h1 align="center">
	<br>
	 🌼
	<br>
	<br>
	Zinnia CLI
	<br>
	<br>
	<br>
</h1>

[![crates](https://img.shields.io/crates/v/zinnia.svg)](https://crates.io/crates/zinnia)

Zinnia is a sandboxed and resource-limited runtime for distributed workers. This crate provides the
`zinnia` executable to run modules locally, e.g. while developing.

**CAUTION: This crate is no longer maintained.**

## Installation

You can download the `zinnia` binary from
[our GitHub Releases](https://github.com/filecoin-station/zinnia/releases/latest).

| OS      | Platform      | Filename                                                                                                                   |
| ------- | ------------- | -------------------------------------------------------------------------------------------------------------------------- |
| Windows | Intel, 64bit  | [zinnia-windows-x64.zip](https://github.com/filecoin-station/zinnia/releases/latest/download/zinnia-windows-x64.zip)       |
| macOS   | Intel, 64bit  | [zinnia-macos-x64.zip](https://github.com/filecoin-station/zinnia/releases/latest/download/zinnia-macos-x64.zip)           |
| macOS   | Apple Silicon | [zinnia-macos-arm64.zip](https://github.com/filecoin-station/zinnia/releases/latest/download/zinnia-macos-arm64.zip)       |
| Linux   | Intel, 64bit  | [zinnia-linux-x64.tar.gz](https://github.com/filecoin-station/zinnia/releases/latest/download/zinnia-linux-x64.tar.gz)     |
| Linux   | ARM, 64bit    | [zinnia-linux-arm64.tar.gz](https://github.com/filecoin-station/zinnia/releases/latest/download/zinnia-linux-arm64.tar.gz) |

### Build from source

If you have Rust tooling installed on your machine (see
[Install Rust](https://www.rust-lang.org/tools/install)), you can build & install Zinnia from the
source code.

**Rust version requirement:** Zinnia requires Rust toolchain version 1.85. If you have `rustup`
installed, you can install this specific version using:

```sh
rustup install 1.85
rustup default 1.85
```

In addition to the Rust build toolchain, you also need Go installed. See
[Go Downloads](https://go.dev/dl/). The minimum required version of Go is 1.22.

On Windows, Go uses `gcc` to create C libraries. Go recommends installing
[TDM GCC](https://jmeubank.github.io/tdm-gcc/).

```sh
$ cargo install zinnia
```

## Basic use

### Run a JavaScript module

```
zinnia run my-module.js
```

See [Building Modules](./docs/building-modules.md) for how to write new modules for Filecoin
Station.

### Run a Rust module

We have decided to put Rust/WASM modules on hold for now.
