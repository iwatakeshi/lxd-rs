# lxd-rs

A comprehensive Rust library for controlling LXD - Linux Containers and Virtual Machines.

[![crates.io](https://img.shields.io/crates/v/lxd.svg)](https://crates.io/crates/lxd)
[![docs.rs](https://docs.rs/lxd/badge.svg)](https://docs.rs/lxd)

## Project Structure

```
lxd-rs/
├── crates/
│   ├── lxd-types/       # Type definitions (210+ types from OpenAPI spec)
│   ├── lxd-client/      # Async REST API client (Unix socket & HTTPS)
│   └── lxd/             # Umbrella crate re-exporting all functionality
└── codegen/             # OpenAPI code generator
```

## Features

- **Async/await** support with Tokio
- **Unix socket** communication (default, no network overhead)
- **HTTPS** support for remote servers
- **Type-safe** API with 210+ types generated from LXD OpenAPI spec
- Direct communication with LXD daemon

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
lxd = "0.2"
tokio = { version = "1.0", features = ["full"] }
```

Or for individual crates:

```toml
[dependencies]
lxd-client = "0.2"  # Just the REST client
lxd-types = "0.2"   # Just the type definitions
```

### Feature Flags

- `generated` - Use 210+ types auto-generated from LXD OpenAPI spec (all fields are `Option<T>`)
- Default - Use hand-crafted types (more ergonomic with proper defaults)

```toml
# To use generated types:
[dependencies]
lxd = { version = "0.2", features = ["generated"] }
```

## Quick Start

### Connect via Unix Socket

```rust
use lxd::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to local LXD via Unix socket
    let client = Client::new_unix_socket()?;
    
    // Get server info
    let server = client.get_server().await?;
    println!("LXD API version: {}", server.api_version);
    
    // List instances
    let instances = client.list_instances_full().await?;
    for instance in instances {
        println!("{}: {}", instance.name, instance.status);
    }
    
    // Start an instance
    client.start_instance("my-container").await?;
    
    Ok(())
}
```

### Connect via HTTPS

```rust
use lxd::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_https(
        "https://lxd.example.com:8443",
        "/path/to/client.crt",
        "/path/to/client.key",
    )?;
    
    let server = client.get_server().await?;
    println!("LXD API version: {}", server.api_version);
    
    Ok(())
}
```

## API Coverage

| Category | Status | Operations |
|----------|--------|------------|
| Server | ✅ | Get info, API version |
| Instances | ✅ | List, Get, Create, Update, Delete, State (start/stop/restart) |
| Images | ✅ | List, Get, Create, Update, Delete |
| Networks | ⏳ | Planned |
| Storage | ⏳ | Planned |
| Profiles | ⏳ | Planned |
| Projects | ⏳ | Planned |
| Cluster | ⏳ | Planned |

## Generated Types

When using the `generated` feature, you get 210+ types covering:

- **Instances** (29 types): Instance, InstanceState, InstanceSnapshot, etc.
- **Networks** (37 types): Network, NetworkLease, NetworkACL, etc.
- **Storage** (25 types): StoragePool, StorageVolume, StorageBucket, etc.
- **Resources** (36 types): ResourcesCPU, ResourcesMemory, ResourcesGPU, etc.
- **Cluster** (16 types): ClusterMember, ClusterGroup, etc.
- And more...

## Code Generation

Types are generated from the official LXD OpenAPI specification:

```bash
cd codegen
cargo run
```

## Requirements

- **LXD daemon** running on Linux
- **Unix socket access**: `/var/snap/lxd/common/lxd/unix.socket` (snap) or `/var/lib/lxd/unix.socket`
- For HTTPS: LXD configured with `core.https_address` and client certificates

### Unix Socket Permissions

```bash
sudo usermod -aG lxd $USER
newgrp lxd
```

### HTTPS Setup

```bash
lxc config set core.https_address "[::]:8443"
```

## Platform Support

| Platform | Support |
|----------|---------|
| Linux | ✅ Full support |
| WSL2 | ✅ Full support |
| Windows | ❌ LXD requires Linux kernel |
| macOS | ❌ LXD requires Linux kernel |

## Testing

```bash
# Run tests
cargo test

# Run with generated types
cargo test --features generated

# Run example
cargo run -p lxd --example unix_socket
```

## License

MIT

## Credits

Originally created by Jeremy Soller. REST API and modernization by the community.

## References

- [LXD Documentation](https://documentation.ubuntu.com/lxd/)
- [LXD REST API](https://documentation.ubuntu.com/lxd/en/latest/rest-api/)
- [LXD GitHub](https://github.com/canonical/lxd)
