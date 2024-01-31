# df-steam-hook-rs

[![Build](https://github.com/dfint/df-steam-hook-rs/actions/workflows/build.yml/badge.svg)](https://github.com/dfint/df-steam-hook-rs/actions/workflows/build.yml)

Re-implementation of [df-steam-hook](https://github.com/dfint/df-steam-hook) in Rust.

Supports Windows and Linux versions of Dwarf Fortress, including [classic](http://www.bay12games.com/dwarves/), [steam](https://store.steampowered.com/app/975370/Dwarf_Fortress/) and [itch.io](https://kitfoxgames.itch.io/dwarf-fortress) versions.

Implemented:

- using of config/offsets files
- using dictionary from csv
- translation hooks
- text entry hooks (e.g. search)

## Build and run

### Install rustc, toolchain by using rustup

For installation of rust and cargo [see the link](https://doc.rust-lang.org/cargo/getting-started/installation.html).

> [!NOTE]  
> The use of system `rustc` and `cargo` often leads to build errors.

On Linux, after the installation use the following command:
```
source "$HOME/.cargo/env"
```
There's no `source` command on Windows, just install `rustup`, it will add cargo to the environment variables (follow the instructions at the link at the beginning of this section).

Then install `nightly-2022-11-06` toolchain using `rustup`:

```shell
rustup install nightly-2022-11-06-x86_64-unknown-linux-gnu
```

### Build

```shell
cargo +nightly-2022-11-06 build --release
```

### Prepare the game

Copy:
* `target/release/libdfint_hook.so` to `libdfhooks.so` in the game's directory on Linux
* or `target/release/dfint_hook.dll` to `dfhooks.dll` on Windows
* [font](https://github.com/dfint/update-data/tree/main/store/fonts) to `data/art/curses_640x300.png`
* [encoding](https://github.com/dfint/update-data/tree/main/store/encodings) to `dfint-data/encoding.toml`
* [offsets](https://github.com/dfint/update-data/tree/main/store/offsets) to `dfint-data/offsets.toml`
* [dictionary](https://github.com/dfint/autobuild/tree/main/translation_build/csv/) to `dfint-data/dictionary.csv`
* [config](https://github.com/dfint/update-data/blob/main/store/config.toml) to `dfint-data/config.toml`

### Launch

Run (on Linux)
```shell
./dwarfort
```
or (on Windows)
```shell
Dwarf Fortress.exe
```
or just double click the executable file of the game (or run it from the steam client, for example).

By default, the log is written to the file: `dfint-data/dfint-log.log`
