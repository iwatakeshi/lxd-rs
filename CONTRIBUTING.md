# Contributing to lxd-rs

Thank you for your interest in contributing to lxd-rs! This document provides guidelines and instructions for contributing.

## Project Structure

```
lxd-rs/
├── Cargo.toml              # Workspace configuration
├── crates/
│   ├── lxd/                # Umbrella crate (use this)
│   │   ├── src/lib.rs      # Re-exports from other crates
│   │   └── examples/       # Usage examples
│   ├── lxd-client/         # REST API client
│   │   ├── src/
│   │   │   ├── lib.rs      # Client implementation
│   │   │   └── transport.rs # Unix socket & HTTPS transports
│   │   └── tests/          # Integration tests
│   └── lxd-types/          # API type definitions
│       └── src/
│           ├── lib.rs      # Type re-exports
│           └── generated/  # Auto-generated from OpenAPI
└── codegen/                # OpenAPI code generator
    ├── src/main.rs         # Generator implementation
    └── swagger.json        # LXD API specification
```

## Getting Started

### Prerequisites

- Rust 1.70+ (2021 edition)
- LXD installed and running (for integration tests)
- Access to the LXD Unix socket (`/var/snap/lxd/common/lxd/unix.socket`)

### Building

```bash
# Build all crates
cargo build --workspace

# Run tests (requires LXD)
cargo test --workspace

# Run clippy
cargo clippy --workspace --all-targets
```

### Running Examples

```bash
# Unix socket example (requires LXD)
cargo run --package lxd --example unix_socket
```

## Development Workflow

### Working with Generated Types

The types in `lxd-types` are generated from the LXD OpenAPI specification:

```bash
cd codegen
cargo run
```

This regenerates all types in `crates/lxd-types/src/generated/`.

**Note:** Manual edits to generated files will be lost. If you need to modify type generation, edit `codegen/src/main.rs`.

### Adding New API Methods

1. Add the method to `crates/lxd-client/src/lib.rs`
2. Use types from `lxd-types` for request/response bodies
3. Add integration tests in `crates/lxd-client/tests/`

### Transport Layer

The client supports two transports:

- **UnixSocket** - For local LXD connections (default, recommended)
- **Https** - For remote LXD servers with TLS certificates

Both implement the same API through the `Transport` enum in `lxd-client`.

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- All public items should have documentation
- Use `thiserror` for error types
- Prefer `async`/`await` over blocking operations

## Testing

```bash
# Run all tests
cargo test --workspace

# Run with output
cargo test --workspace -- --nocapture

# Run specific test
cargo test --package lxd-client integration
```

**Note:** Integration tests require a running LXD instance with socket access.

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and clippy
5. Commit with descriptive messages
6. Push and create a Pull Request

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
