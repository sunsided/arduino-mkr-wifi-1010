[private]
help:
    @just --list --unsorted

# Run pre-commit hooks on all files.
pre-commit:
    @pre-commit run --all-files

# Build all examples
build:
    @cargo build --examples --features=usb,unproven

# Run cargo-autoinherit on the workspace.
autoinherit:
    # cargo install --locked cargo-autoinherit
    cargo autoinherit

# Reload and apply the installed udev rules
relad-udev-rules:
    sudo udevadm control --reload-rules
    sudo udevadm trigger

# Get device information using bossac
device-info:
    bossac -i -d --usb-port

# Execute basic build and flash steps
flash:
    cargo build --release --example blinky_basic
    arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/examples/blinky_basic target/blinky_basic.bin
    bossac -i -d --usb-port -e -w -v -R target/blinky_basic.bin
