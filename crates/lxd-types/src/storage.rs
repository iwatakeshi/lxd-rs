//! Storage types for LXD API

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// StoragePool represents an LXD storage pool
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoragePool {
    /// Pool name
    pub name: String,

    /// Pool description
    #[serde(default)]
    pub description: String,

    /// Pool driver (dir, btrfs, lvm, zfs, ceph, etc.)
    pub driver: String,

    /// Pool configuration
    #[serde(default)]
    pub config: BTreeMap<String, String>,

    /// Pool status
    #[serde(default)]
    pub status: String,

    /// Resources using this pool
    #[serde(default)]
    pub used_by: Vec<String>,

    /// Locations (for clusters)
    #[serde(default)]
    pub locations: Vec<String>,
}

/// Request to create a new storage pool
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolsPost {
    /// Pool name
    pub name: String,

    /// Pool driver
    pub driver: String,

    /// Pool description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Pool configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
}

impl StoragePoolsPost {
    /// Create a new storage pool request
    pub fn new(name: impl Into<String>, driver: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            driver: driver.into(),
            description: None,
            config: None,
        }
    }

    /// Create a directory storage pool
    pub fn dir(name: impl Into<String>) -> Self {
        Self::new(name, "dir")
    }

    /// Create a btrfs storage pool
    pub fn btrfs(name: impl Into<String>) -> Self {
        Self::new(name, "btrfs")
    }

    /// Create a zfs storage pool
    pub fn zfs(name: impl Into<String>) -> Self {
        Self::new(name, "zfs")
    }

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the configuration
    pub fn with_config(mut self, config: BTreeMap<String, String>) -> Self {
        self.config = Some(config);
        self
    }
}

/// Request to update a storage pool
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StoragePoolPut {
    /// Pool description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Pool configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
}

/// StorageVolume represents a storage volume in a pool
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageVolume {
    /// Volume name
    pub name: String,

    /// Volume type (custom, container, virtual-machine, image)
    #[serde(rename = "type", default)]
    pub volume_type: String,

    /// Volume description
    #[serde(default)]
    pub description: String,

    /// Volume configuration
    #[serde(default)]
    pub config: BTreeMap<String, String>,

    /// Volume content type (filesystem or block)
    #[serde(default)]
    pub content_type: String,

    /// Resources using this volume
    #[serde(default)]
    pub used_by: Vec<String>,

    /// Location (for clusters)
    #[serde(default)]
    pub location: String,
}

/// Request to create a new storage volume
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageVolumesPost {
    /// Volume name
    pub name: String,

    /// Volume type
    #[serde(rename = "type", default)]
    pub volume_type: String,

    /// Volume description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Volume configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,

    /// Content type (filesystem or block)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl StorageVolumesPost {
    /// Create a new custom volume
    pub fn custom(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            volume_type: "custom".to_string(),
            description: None,
            config: None,
            content_type: None,
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the size
    pub fn with_size(mut self, size: impl Into<String>) -> Self {
        let mut config = self.config.unwrap_or_default();
        config.insert("size".to_string(), size.into());
        self.config = Some(config);
        self
    }

    /// Set as block content type
    pub fn block(mut self) -> Self {
        self.content_type = Some("block".to_string());
        self
    }
}

/// Request to update a storage volume
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageVolumePut {
    /// Volume description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Volume configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
}
