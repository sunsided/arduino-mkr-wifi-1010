[private]
help:
    @just --list --unsorted

# Run pre-commit hooks on all files.
pre-commit:
    @pre-commit run --all-files

# Build all targets.
build WHAT="--all-targets":
    @cargo build {{ WHAT }}
