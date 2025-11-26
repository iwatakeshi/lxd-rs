# Architecture

This document describes the architecture of the lxd-rs library.

## Overview

lxd-rs is a Rust client library for the [LXD](https://linuxcontainers.org/lxd/) container hypervisor. It communicates with LXD via its REST API over Unix sockets or HTTPS.

```
┌─────────────────────────────────────────────────────────┐
│                     User Application                     │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                      lxd (umbrella)                      │
│                   Re-exports everything                  │
└─────────────────────────────────────────────────────────┘
                            │
              ┌─────────────┴─────────────┐
              ▼                           ▼
┌─────────────────────────┐   ┌─────────────────────────┐
│       lxd-client        │   │       lxd-types         │
│     REST API Client     │   │    Type Definitions     │
└─────────────────────────┘   └─────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────────────────────┐
│                       Transport                          │
│              UnixSocket  │  Https                        │
└─────────────────────────────────────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────────────────────┐
│                      LXD Daemon                          │
└─────────────────────────────────────────────────────────┘
```

## Crates

### `lxd` (Umbrella Crate)

The main entry point for users. Re-exports types and client from the other crates.

```rust
use lxd::Client;
use lxd::types::Instance;
```

### `lxd-client`

The REST API client implementation.

**Key Components:**

- `Client` - Main client struct with async methods for each LXD endpoint
- `Transport` - Enum-based transport abstraction
  - `UnixSocket` - Connects via Unix domain socket (recommended for local)
  - `Https` - Connects via HTTPS with TLS certificates

**Design Decisions:**

- Uses enum dispatch instead of trait objects for transport (dyn-compatible)
- Async-first with Tokio runtime
- All operations return `Result<T, Error>` with typed responses

### `lxd-types`

Type definitions matching the LXD API.

**Structure:**
- `src/generated/` - Types auto-generated from OpenAPI spec
- `src/lib.rs` - Re-exports and any manual type additions

**Generated Modules:**
- `instances` - Container/VM definitions
- `images` - Image metadata
- `networks` - Network configuration
- `storage` - Storage pools and volumes
- `profiles` - Instance profiles
- `projects` - Project definitions
- `operations` - Async operation tracking
- `server` - Server information
- And more...

## Code Generation

The `codegen/` directory contains a custom OpenAPI code generator.

### Why Custom Generator?

1. LXD uses Swagger 2.0 (not OpenAPI 3.0)
2. Need custom handling for `x-go-name` and LXD-specific patterns
3. Want idiomatic Rust output with proper serde annotations

### Generator Features

- Parses Swagger 2.0 JSON specification
- Handles nested object definitions
- Generates proper Rust types with:
  - `serde::Serialize` and `Deserialize`
  - `Clone`, `Debug`, `Default` derives
  - Optional fields with `Option<T>`
  - Doc comments from descriptions

### Running the Generator

```bash
cd codegen
cargo run
```

Output goes to `crates/lxd-types/src/generated/`.

## Transport Layer

### Unix Socket (Recommended)

```rust
let transport = Transport::unix_socket("/var/snap/lxd/common/lxd/unix.socket");
let client = Client::new(transport);
```

- Default for local LXD installations
- No authentication required (uses socket permissions)
- Uses `hyper-util` with Unix socket connector

### HTTPS

```rust
let transport = Transport::https(
    "https://lxd.example.com:8443",
    client_cert,
    client_key,
)?;
let client = Client::new(transport);
```

- For remote LXD servers
- Requires TLS client certificate authentication
- Uses `rustls` for TLS

## Error Handling

All client methods return `Result<T, lxd_client::Error>`.

Error types include:
- `Error::Http` - HTTP request failures
- `Error::Api` - LXD API error responses
- `Error::Transport` - Connection issues
- `Error::Serialization` - JSON parsing failures

## Async Runtime

The library uses Tokio as its async runtime:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::unix_socket_default()?;
    let instances = client.list_instances().await?;
    Ok(())
}
```

## Future Improvements

- [ ] WebSocket support for operations and events
- [ ] Connection pooling for HTTPS
- [ ] More comprehensive API coverage
- [ ] Builder patterns for complex requests
- [ ] Retry logic with backoff
