# Rawk development tasks
# ++++++++++++++++++++++

default:
    @just --list

# Build project
build:
    cargo build

# Run CLI
run *ARGS:
    cargo run -- {{ARGS}}

# Check code
check:
    cargo check

# Run tests
test:
    cargo test

# Format code
fmt:
    cargo fmt

# Lint code
lint:
    cargo clippy

# Clean build
clean:
    cargo clean
