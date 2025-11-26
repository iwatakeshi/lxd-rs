# lxd-rs

A native Rust client library for [LXD](https://canonical.com/lxd) – the modern container and virtual machine manager.

[![Crates.io](https://img.shields.io/crates/v/lxd.svg)](https://crates.io/crates/lxd)
[![Documentation](https://docs.rs/lxd/badge.svg)](https://docs.rs/lxd)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Features

- 🚀 **Async/await** – Built on Tokio for high-performance async I/O
- 🔌 **Unix Socket** – Zero-overhead local connections (default)
- 🔒 **HTTPS + TLS** – Secure remote connections with client certificates
- 📦 **210+ Types** – Complete type coverage generated from LXD's Swagger spec
- 🦀 **Pure Rust** – No CLI wrappers or shell commands

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
lxd = "0.2"
tokio = { version = "1", features = ["full"] }
```

### Connect to LXD

```rust
use lxd::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect via Unix socket (default)
    let client = Client::new_unix_socket()?;

    // Get server info
    let server = client.get_server().await?;
    println!("LXD API v{}", server.api_version);

    // List all instances
    let instances = client.list_instances_full().await?;
    for instance in instances {
        println!("{}: {}", instance.name, instance.status);
    }

    Ok(())
}
```

### Create a Container

```rust
use lxd::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_unix_socket()?;

    // Create from an image alias
    let source = InstanceSource::from_image("ubuntu/22.04");
    let request = InstancesPost::new("my-container", source);

    let operation = client.create_instance(&request).await?;
    
    // Wait for the operation to complete
    client.wait_operation(&operation.id, None).await?;
    
    // Start the container
    client.start_instance("my-container").await?;

    Ok(())
}
```

### Connect to Remote LXD Server

```rust
use lxd::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_https(
        "https://lxd.example.com:8443",
        "/path/to/client.crt",
        "/path/to/client.key",
    )?;

    let instances = client.list_instances_full().await?;
    println!("Remote server has {} instances", instances.len());

    Ok(())
}
```

### Configure Timeouts and Retries

```rust
use lxd::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_unix_socket()?
        .with_timeout(Duration::from_secs(60))
        .with_retries(5);

    // Or use the builder for more control
    let config = ClientConfig::new()
        .with_timeout(Duration::from_secs(120))
        .with_retries(3)
        .with_retry_delay(Duration::from_millis(200));

    let client = Client::new_unix_socket()?.with_config(config);
    
    Ok(())
}
```

## API Coverage

### Instances (Containers & VMs)

| Method | Description |
|--------|-------------|
| `list_instances()` | List instance URLs |
| `list_instances_full()` | List instances with full details |
| `get_instance(name)` | Get instance details |
| `create_instance(request)` | Create a new instance |
| `update_instance(name, request)` | Update instance config |
| `delete_instance(name)` | Delete an instance |
| `get_instance_state(name)` | Get instance state |
| `start_instance(name)` | Start an instance |
| `stop_instance(name, force)` | Stop an instance |
| `restart_instance(name)` | Restart an instance |
| `freeze_instance(name)` | Freeze (pause) an instance |
| `unfreeze_instance(name)` | Unfreeze an instance |

### Instance Snapshots

| Method | Description |
|--------|-------------|
| `list_instance_snapshots(instance)` | List snapshot URLs |
| `list_instance_snapshots_full(instance)` | List snapshots with details |
| `get_instance_snapshot(instance, name)` | Get snapshot details |
| `create_instance_snapshot(instance, request)` | Create a snapshot |
| `delete_instance_snapshot(instance, name)` | Delete a snapshot |
| `restore_instance_snapshot(instance, name)` | Restore from snapshot |

### Images

| Method | Description |
|--------|-------------|
| `list_images()` | List image fingerprints |
| `list_images_full()` | List images with full details |
| `get_image(fingerprint)` | Get image details |
| `create_image(request)` | Import an image |
| `update_image(fingerprint, request)` | Update image properties |
| `delete_image(fingerprint)` | Delete an image |

### Networks

| Method | Description |
|--------|-------------|
| `list_networks()` | List network URLs |
| `list_networks_full()` | List networks with details |
| `get_network(name)` | Get network details |
| `create_network(request)` | Create a network |
| `update_network(name, request)` | Update network config |
| `delete_network(name)` | Delete a network |

### Storage Pools

| Method | Description |
|--------|-------------|
| `list_storage_pools()` | List storage pool URLs |
| `list_storage_pools_full()` | List pools with details |
| `get_storage_pool(name)` | Get pool details |
| `create_storage_pool(request)` | Create a storage pool |
| `update_storage_pool(name, request)` | Update pool config |
| `delete_storage_pool(name)` | Delete a storage pool |

### Storage Volumes

| Method | Description |
|--------|-------------|
| `list_storage_volumes(pool)` | List volume URLs |
| `list_storage_volumes_full(pool)` | List volumes with details |
| `get_storage_volume(pool, type, name)` | Get volume details |
| `create_storage_volume(pool, request)` | Create a volume |
| `update_storage_volume(pool, type, name, request)` | Update volume |
| `delete_storage_volume(pool, type, name)` | Delete a volume |

### Profiles

| Method | Description |
|--------|-------------|
| `list_profiles()` | List profile URLs |
| `list_profiles_full()` | List profiles with details |
| `get_profile(name)` | Get profile details |
| `create_profile(request)` | Create a profile |
| `update_profile(name, request)` | Update profile |
| `delete_profile(name)` | Delete a profile |

### Projects

| Method | Description |
|--------|-------------|
| `list_projects()` | List project URLs |
| `list_projects_full()` | List projects with details |
| `get_project(name)` | Get project details |
| `create_project(request)` | Create a project |
| `update_project(name, request)` | Update project |
| `delete_project(name)` | Delete a project |

### Operations

| Method | Description |
|--------|-------------|
| `list_operations()` | List all operations |
| `get_operation(id)` | Get operation details |
| `wait_operation(id, timeout)` | Wait for operation to complete |
| `cancel_operation(id)` | Cancel an operation |

### Server

| Method | Description |
|--------|-------------|
| `get_server()` | Get server information |
| `get_api_version()` | Get API version string |

## Project Structure

```
lxd-rs/
├── crates/
│   ├── lxd/           # Main crate (use this one)
│   ├── lxd-client/    # REST API client
│   └── lxd-types/     # Type definitions
└── codegen/           # Swagger → Rust code generator
```

### Crate Overview

| Crate | Description |
|-------|-------------|
| [`lxd`](https://crates.io/crates/lxd) | Umbrella crate – re-exports client and types |
| [`lxd-client`](https://crates.io/crates/lxd-client) | Async REST client with Unix socket & HTTPS support |
| [`lxd-types`](https://crates.io/crates/lxd-types) | 210+ type definitions from LXD's Swagger spec |

## Feature Flags

| Feature | Description |
|---------|-------------|
| `generated` | Use all 210+ auto-generated types (fields are `Option<T>`) |
| *(default)* | Use hand-crafted types with sensible defaults |

```toml
# Use generated types for maximum API coverage
lxd = { version = "0.2", features = ["generated"] }
```

## Requirements

- **Rust 1.75+** (async fn in traits)
- **LXD 4.0+** (REST API v1.0)
- **Linux** (LXD only runs on Linux)

### Permissions

To access the LXD Unix socket, your user must be in the `lxd` group:

```bash
sudo usermod -aG lxd $USER
newgrp lxd  # or log out and back in
```

## Error Handling

All client methods return `Result<T, lxd_client::Error>`:

```rust
use lxd::prelude::*;

match client.get_instance("nonexistent").await {
    Ok(instance) => println!("Found: {}", instance.name),
    Err(ClientError::Api { code, message }) => {
        eprintln!("LXD error {}: {}", code, message);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Examples

See the [`examples/`](crates/lxd/examples) directory:

```bash
# List instances via Unix socket
cargo run --package lxd --example unix_socket
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development

```bash
# Build all crates
cargo build --workspace

# Run tests (requires LXD)
cargo test --workspace

# Run clippy
cargo clippy --workspace --all-targets

# Regenerate types from Swagger spec
cd codegen && cargo run
```

## License

MIT License – see [LICENSE](LICENSE) for details.

## Related Projects

- [LXD](https://canonical.com/lxd) – The container/VM hypervisor
- [lxc](https://github.com/lxc/lxc) – Low-level container runtime
- [incus](https://github.com/lxc/incus) – LXD community fork
