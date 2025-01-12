# Arduino MKR-WIFI-1010 Playground

## Table of Contents

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

- [Pre-commit hooks](#pre-commit-hooks)
- [Requirements](#requirements)
- [Arduino burning bootloader](#arduino-burning-bootloader)

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

Install `bossac`, the Flash programmer for Atmel SAM devices:

```shell
sudo apt install bossa-cli
```


## Arduino burning bootloader

These builds overwrite the Arduino bootloader on the MKR WiFi 1010.
In theory, flashing the application to `0x2000` (and potentially adjusting
the interrupt vector table offset register accordingly) should do the trick;
in practice, this always locked up the core for me.

To restore the bootloader, I used a cheap [DAP-LINK probe](https://amzn.to/3Px1DEw)
with pin soldered to the SWD pad on the bottom of the board and have
the Arduino IDE believe it talks to a Atmel SAM-ICE. Here's the output of that:

```plain
Open On-Chip Debugger 0.10.0+dev-gf0767a31 (2018-06-11-13:40)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
debug_level: 0
0
none separate
adapter speed: 400 kHz
cortex_m reset_config sysresetreq
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x000002f8 msp: 0x20007fd0
target halted due to debug-request, current mode: Thread
xPSR: 0x01000000 pc: 0x000000b0 msp: 0x20008000
** Programming Started **
auto erase enabled
wrote 8192 bytes from file /home/markus/.arduino15/packages/arduino/hardware/samd/1.8.14/bootloaders/mkrwifi1010/samd21_sam_ba_arduino_mkrwifi1010.bin in 2.584113s (3.096 KiB/s)
** Programming Finished **
** Verify Started **
verified 7984 bytes in 0.788973s (9.882 KiB/s)
** Verified OK **
** Resetting Target **
shutdown command invoked
```
