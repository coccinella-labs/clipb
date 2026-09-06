<p align="center">
  <img src="https://raw.githubusercontent.com/Coccinella-Labs/clipb/main/.github/assets/thumbnail.png" alt="clipb" width="100%">
</p>

# clipb

[![Release](https://img.shields.io/github/v/release/libnudget/clipb?logo=github&label=latest)](https://github.com/libnudget/clipb/releases)

A lightweight clipboard utility for developers.

clipb copies file paths to the system clipboard without leaving your
terminal. It prints the copied paths back to stdout so you can chain it
into other commands, and it handles spaces, symlinks, and relative paths
correctly.

## Installation

```sh
cargo install --git https://github.com/libnudget/clipb
```

## Usage

Copy one or more paths:

```sh
clipb copy src/main.rs
clipb copy "$PWD/src" ~/.config/zed/settings.json
```

Print the current clipboard:

```sh
clipb paste
```

Clear the clipboard:

```sh
clipb clear
```

## Development

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## License

MIT
