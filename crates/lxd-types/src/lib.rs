//! # LXD Types
//!
//! Type definitions for the LXD REST API, generated from the Swagger 2.0 specification.
//!
//! This crate provides strongly-typed Rust structs for all LXD API resources including:
//! - Instances (containers and virtual machines)
//! - Images
//! - Networks
//! - Storage pools and volumes
//! - Profiles and projects
//! - Cluster configuration
//! - Operations
//!
//! ## Features
//!
//! - `generated` - Include all 210+ types generated from Swagger spec
//!
//! ## Example
//!
//! ```rust
//! use lxd_types::*;
//! ```

// Generated types from Swagger 2.0 spec (210+ types)
// When enabled, these take precedence
#[cfg(feature = "generated")]
pub mod generated;

#[cfg(feature = "generated")]
pub use generated::*;

// Core types - only available when generated feature is NOT enabled
// This avoids conflicts between hand-written and generated types
#[cfg(not(feature = "generated"))]
mod common;
#[cfg(not(feature = "generated"))]
mod images;
#[cfg(not(feature = "generated"))]
mod instances;
#[cfg(not(feature = "generated"))]
mod networks;
#[cfg(not(feature = "generated"))]
mod operations;
#[cfg(not(feature = "generated"))]
mod profiles;
#[cfg(not(feature = "generated"))]
mod projects;
#[cfg(not(feature = "generated"))]
mod snapshots;
#[cfg(not(feature = "generated"))]
mod storage;

#[cfg(not(feature = "generated"))]
pub use common::*;
#[cfg(not(feature = "generated"))]
pub use images::*;
#[cfg(not(feature = "generated"))]
pub use instances::*;
#[cfg(not(feature = "generated"))]
pub use networks::*;
#[cfg(not(feature = "generated"))]
pub use operations::*;
#[cfg(not(feature = "generated"))]
pub use profiles::*;
#[cfg(not(feature = "generated"))]
pub use projects::*;
#[cfg(not(feature = "generated"))]
pub use snapshots::*;
#[cfg(not(feature = "generated"))]
pub use storage::*;

// Response types are always available as they're not in the Swagger spec
mod responses;
pub use responses::*;
