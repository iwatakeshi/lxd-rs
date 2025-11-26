//! Common types used across the LXD API

use std::collections::BTreeMap;

/// Device configuration
pub type DeviceConfig = BTreeMap<String, String>;

/// Instance configuration
pub type Config = BTreeMap<String, String>;

/// Metadata map
pub type Metadata = BTreeMap<String, serde_json::Value>;
