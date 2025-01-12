[private]
help:
    @just --list --unsorted

# Run pre-commit hooks on all files.
pre-commit:
    @pre-commit run --all-files


# Run basic build
build PROGRAM="blinky_basic":
    cargo build --features=usb,unproven --release --example {{PROGRAM}}

# Build all examples
build-all:
    cargo build --release --features=usb,unproven --examples

# Run cargo-autoinherit on the workspace.
autoinherit:
    # cargo install --locked cargo-autoinherit
    cargo autoinherit

# Reload and apply the installed udev rules
relad-udev-rules:
    sudo udevadm control --reload-rules
    sudo udevadm trigger

# Get device information using bossac
device-info PORT="/dev/ttyS0":
    bossac -i -d --usb-port --port {{PORT}}

# Execute basic build and flash steps
flash PROGRAM="blinky_basic":
    cargo flash --features=usb,unproven --release --example {{PROGRAM}}
    # cargo build --release --example blinky_basic
    # arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/examples/blinky_basic target/blinky_basic.bin
    # bossac -i -d --usb-port -e -w -v -R -p /dev/ttyACM0 target/blinky_basic.bin

# Run basic build, flash and attach
run PROGRAM="blinky_basic":
    cargo run --features=usb,unproven --release --example {{PROGRAM}}
