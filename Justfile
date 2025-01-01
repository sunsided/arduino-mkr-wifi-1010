[private]
help:
    @just --list --unsorted

# Run pre-commit hooks on all files.
pre-commit:
    @pre-commit run --all-files

# Build all examples
build:
    @cargo build --examples --features=usb,unproven

# Runs cargo-autoinherit on the workspace.
autoinherit:
    # cargo install --locked cargo-autoinherit
    cargo autoinherit
