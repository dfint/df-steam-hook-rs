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

### build

```shell
cargo +nightly-2022-11-06 build --release
```
