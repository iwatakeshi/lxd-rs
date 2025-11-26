//! # LXD
//!
//! A comprehensive Rust library for controlling LXD containers and virtual machines.
//!
//! This crate provides:
//! - **REST API client** - Async API for Unix socket and HTTPS
//! - **Type definitions** - Strongly-typed API types (210+)
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use lxd::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect via Unix socket (default for local LXD)
//!     let client = Client::new_unix_socket()?;
//!     
//!     // List all instances
//!     let instances = client.list_instances_full().await?;
//!     for instance in instances {
//!         println!("{}: {}", instance.name, instance.status);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Connecting via HTTPS
//!
//! ```rust,no_run
//! use lxd::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new_https(
//!         "https://lxd.example.com:8443",
//!         "/path/to/client.crt",
//!         "/path/to/client.key",
//!     )?;
//!     
//!     let server = client.get_server().await?;
//!     println!("Connected to LXD API version: {}", server.api_version);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Feature Flags
//!
//! - `generated` - Use 210+ types generated from LXD Swagger spec

#![deny(missing_docs)]

// Re-export types
pub use lxd_types::*;

// Re-export client module
pub mod client {
    //! REST API client module
    pub use lxd_client::*;
}

pub use lxd_client::{Client, Error as ClientError, Result as ClientResult, Transport, ClientConfig, ClientBuilder};

/// Prelude module for convenient imports
///
/// ```rust,no_run
/// use lxd::prelude::*;
/// ```
pub mod prelude {
    pub use lxd_types::*;
    pub use lxd_client::{Client, Error as ClientError, Result as ClientResult, ClientConfig, ClientBuilder};
}
