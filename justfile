# ++++++++++++++++++++++++++++
# Rawk Development Justfile
# ++++++++++++++++++++++++++++
# Modern ML project templates that rawk

# Show all available commands
default:
    @just --list

# ============================================
# Building
# ============================================

# Build project (debug mode)
build:
    @echo "🔨 Building Rawk..."
    cargo build
    @echo "✓ Build complete"

# Build optimized release version
build-release:
    @echo "Building release version..."
    cargo build --release
    @echo "✓ Release build complete: target/release/rawk"

# Check code without building
check:
    @echo "Checking code..."
    cargo check
    @echo "✓ Code check passed"

# ============================================
# Running
# ============================================

# Run Rawk CLI
run *ARGS:
    cargo run -- {{ARGS}}

# List available templates
list:
    cargo run -- list

# Show template info
info TEMPLATE:
    cargo run -- info {{TEMPLATE}}

# Search templates
search QUERY:
    cargo run -- search {{QUERY}}

# ============================================
# Testing
# ============================================

# Run all tests
test:
    @echo "Running tests..."
    cargo test
    @echo "✓ All tests passed"

# Run tests with output
test-verbose:
    @echo "Running tests (verbose)..."
    cargo test -- --nocapture

# Run tests with coverage (requires cargo-tarpaulin)
test-coverage:
    @echo "Running tests with coverage..."
    cargo tarpaulin --out Html --output-dir coverage
    @echo "✓ Coverage report: coverage/index.html"

# Run specific test
test-one TEST:
    cargo test {{TEST}} -- --nocapture

# ============================================
# Demo & Development Testing
# ============================================

# Create a demo project to test
demo:
    @echo "Creating demo project..."
    cargo run -- new demo-ml-project --template ml/simple-ml
    @echo "✓ Demo project created: demo-ml-project/"
    @echo "  cd demo-ml-project"
    @echo "  ./setup.sh"

# Create demo and enter it
demo-enter: demo
    @echo "Entering demo project..."
    cd demo-ml-project && exec $$SHELL

# Clean up demo projects
clean-demos:
    @echo "🧹 Cleaning demo projects..."
    rm -rf demo-ml-project test-* my-*
    @echo "✓ Demo projects cleaned"

# ============================================
# Code Quality
# ============================================

# Format code
fmt:
    @echo "Formatting code..."
    cargo fmt
    @echo "✓ Code formatted"

# Check formatting without modifying
fmt-check:
    @echo "Checking code format..."
    cargo fmt -- --check

# Lint code
lint:
    @echo " Linting code..."
    cargo clippy -- -D warnings
    @echo "✓ Linting passed"

# Fix common issues
fix:
    @echo "Fixing common issues..."
    cargo clippy --fix --allow-dirty --allow-staged
    cargo fmt
    @echo "✓ Issues fixed"

# Run all quality checks
quality: fmt-check lint test
    @echo "✅ All quality checks passed!"

# ============================================
# Installation
# ============================================

# Install Rawk locally
install:
    @echo "Installing Rawk..."
    cargo install --path .
    @echo "✓ Rawk installed!"
    @echo "  Try: rawk --version"

# Uninstall Rawk
uninstall:
    @echo " Uninstalling Rawk..."
    cargo uninstall rawk
    @echo "✓ Rawk uninstalled"

# Install in release mode
install-release:
    @echo "Installing Rawk (release)..."
    cargo install --path . --release
    @echo "✓ Rawk installed!"

# ============================================
# Cleaning
# ============================================

# Clean build artifacts
clean:
    @echo "Cleaning build artifacts..."
    cargo clean
    @echo "✓ Build artifacts cleaned"

# Clean everything (builds + demos)
clean-all: clean clean-demos
    @echo "✓ Everything cleaned"

# ============================================
# Development
# ============================================

# Watch for changes and rebuild
watch:
    @echo "Watching for changes..."
    cargo watch -x check -x test

# Watch and run on changes
watch-run:
    @echo "Watching and running..."
    cargo watch -x "run -- list"

# Generate documentation
docs:
    @echo "Generating documentation..."
    cargo doc --no-deps --open
    @echo "✓ Documentation generated"

# ============================================
# Release
# ============================================

# Build and test everything before release
pre-release: clean quality build-release
    @echo "Pre-release checks complete!"
    @echo "  Binary: target/release/rawk"
    @echo "  Version: $(cargo pkgid | cut -d# -f2)"

# Show project info
info-project:
    @echo "Rawk - Modern ML Templates"
    @echo ""
    @echo "Version: $(cargo pkgid | cut -d# -f2)"
    @echo "Rust version: $(rustc --version)"
    @echo "Cargo version: $(cargo --version)"
    @echo ""
    @echo "Templates:"
    @find templates -name "rawk.toml" | wc -l | xargs echo "  Count:"
    @echo ""
    @echo "Lines of code:"
    @find src -name "*.rs" -exec wc -l {} + | tail -1

# ============================================
# Git & GitHub
# ============================================

# Commit all changes
commit MESSAGE:
    git add .
    git commit -m "{{MESSAGE}}"

# Commit and push
push MESSAGE: (commit MESSAGE)
    git push origin main

# Show git status
status:
    @git status

# ============================================
# Benchmarking (future)
# ============================================

# Run benchmarks (requires cargo-criterion)
bench:
    @echo "⚡ Running benchmarks..."
    cargo bench
    @echo "✓ Benchmarks complete"

# ============================================
# Template Development
# ============================================

# Validate all templates
validate-templates:
    @echo "✅ Validating all templates..."
    @for template in templates/*/*/; do \
        if [ -f "$$template/rawk.toml" ]; then \
            echo "Checking $$template"; \
            cargo run -- validate "$$template" || exit 1; \
        fi; \
    done
    @echo "✓ All templates valid"

# Count template files
count-templates:
    @echo "Template Statistics:"
    @echo "  Templates: $(find templates -name 'rawk.toml' | wc -l)"
    @echo "  .jinja files: $(find templates -name '*.jinja' | wc -l)"
    @echo "  Categories: $(ls templates | wc -l)"

# ============================================
# Help
# ============================================

# Show detailed help
help:
    @echo "Rawk Development Commands"
    @echo ""
    @echo "Building:"
    @echo "  just build              Build debug version"
    @echo "  just build-release      Build optimized release"
    @echo "  just check              Quick code check"
    @echo ""
    @echo "Running:"
    @echo "  just run [ARGS]         Run Rawk CLI"
    @echo "  just list               List templates"
    @echo "  just info TEMPLATE      Show template info"
    @echo "  just demo               Create demo project"
    @echo ""
    @echo "Testing:"
    @echo "  just test               Run all tests"
    @echo "  just test-verbose       Run tests with output"
    @echo "  just quality            Run all quality checks"
    @echo ""
    @echo "Code Quality:"
    @echo "  just fmt                Format code"
    @echo "  just lint               Lint code"
    @echo "  just fix                Fix common issues"
    @echo ""
    @echo "Installation:"
    @echo "  just install            Install Rawk locally"
    @echo "  just uninstall          Uninstall Rawk"
    @echo ""
    @echo "Cleaning:"
    @echo "  just clean              Clean build artifacts"
    @echo "  just clean-demos        Clean demo projects"
    @echo "  just clean-all          Clean everything"
    @echo ""
    @echo "Use 'just --list' to see all commands"
