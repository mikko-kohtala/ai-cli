# Makefile for ai-cli
# Run common development tasks

.PHONY: check fmt fmt-fix clippy test build install clean run doctor help

# Run all checks (format, lint, test)
check: fmt clippy test
	@echo "✓ All checks passed!"

# Check formatting
fmt:
	@echo "Checking code formatting..."
	@cargo fmt --all -- --check

# Apply formatting
fmt-fix:
	@echo "Applying code formatting..."
	@cargo fmt --all

# Run clippy linter
clippy:
	@echo "Running clippy..."
	@cargo clippy --all-targets --all-features

# Run tests
test:
	@cargo test

# Build the project
build:
	@cargo build --release

# Install the binary
install: build
	@cargo install --path .

# Clean build artifacts
clean:
	@cargo clean

# Run the application
run:
	@cargo run

# Run doctor diagnostics
doctor:
	@cargo run -- mcp doctor

# Show help
help:
	@echo "Available targets:"
	@echo "  check     - Run all checks (fmt, clippy, test)"
	@echo "  fmt       - Check code formatting"
	@echo "  fmt-fix   - Apply code formatting"
	@echo "  clippy    - Run clippy linter"
	@echo "  test      - Run tests"
	@echo "  build     - Build release binary"
	@echo "  install   - Install binary to system"
	@echo "  clean     - Clean build artifacts"
	@echo "  run       - Run the application"
	@echo "  doctor    - Run doctor diagnostics"
	@echo "  help      - Show this help message"
