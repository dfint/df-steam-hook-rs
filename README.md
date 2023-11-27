# df-steam-hook-rs

[![Build](https://github.com/dfint/df-steam-hook-rs/actions/workflows/build.yml/badge.svg)](https://github.com/dfint/df-steam-hook-rs/actions/workflows/build.yml)

Re-implementation of [df-steam-hook](https://github.com/dfint/df-steam-hook) in
Rust.

Supports:

- Windows (classic/steam/itch.io)
- Linux (classic/steam/itch.io)

Implemented:

- config/offsets files
- dictionary from csv
- translation hooks
- enter string hooks (search)

## Build and run

The instructions provided here are for the Linux, but they will also work on Windows. 
The process for running them is the same.

### Install rustc, toolchain by using rustup

For install RUST and Cargo [see link](https://doc.rust-lang.org/cargo/getting-started/installation.h).

Or type:
```shell
curl https://sh.rustup.rs -sSf | sh
```

NOTE: The use of system rustc and cargo often leads to build errors.

Example:
```
error: no such command: `+nightly-2022-11-06`

  Cargo does not handle `+toolchain` directives.
  Did you mean to invoke `cargo` through `rustup` instead?

```

After loading $HOME/.cargo/env:

```
source "$HOME/.cargo/env"
```

Install nightly-2022-11-06 toolchain from rustup:

```shell
rustup install nightly-2022-11-06-x86_64-unknown-linux-gnu

```

### Build

```shell
cargo +nightly-2022-11-06 build --release
```

### Prepare game

To enable translation, you must change the game settings to your native language.
You can use [classic](http://www.bay12games.com/dwarves/), steam or itch.io version.

Copy:
* target/release/libdfint_hook.so to libdfhooks.so
* [font](https://github.com/dfint/update-data/tree/main/store/fonts) to  ./data/art/curses_640x300.png
* [encoding](https://github.com/dfint/update-data/tree/main/store/encodings) to dfint-data/encoding.toml
* [offsets](https://github.com/dfint/update-data/tree/main/store/offsets) to dfint-data/offsets.toml
* [dictionary](https://github.com/dfint/autobuild/tree/main/translation_build/csv/) to dfint-data/dictionary.csv
* [config](https://github.com/dfint/update-data/blob/main/store/config.toml) to ./dfint-data/config.toml

### Launch

Run
```shell
./dwarfort
```

By default config, the log is written to the file: ./dfint-data/dfint-log.log
