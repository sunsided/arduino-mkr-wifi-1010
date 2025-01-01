# Arduino MKR-WIFI-1010 Playground

## Table of Contents

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

- [Pre-commit hooks](#pre-commit-hooks)
- [Requirements](#requirements)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Pre-commit hooks

```shell
sudo apt install pre-commit
```

After cloning this repository for the first time, change into it and execute

```shell
pre-commit install
```

Pre-commit hooks are then executed on each commit according to the
[.pre-commit-config.yaml](.pre-commit-config-yaml). You can force an offline run by calling

```shell
pre-commit run --all-files
```

## Requirements

Install the toolchain:

```shell
rustup target add thumbv6m-none-eabi
```

Install `probe-rs`:

```shell
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

Install `elf2uf2`:

```shell
sudo apt install libudev-dev
cargo install elf2uf2-rs
```
