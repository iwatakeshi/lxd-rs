//! Instance snapshot types for LXD API

use serde::{Deserialize, Serialize};

/// InstanceSnapshot represents an instance snapshot
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceSnapshot {
    /// Snapshot name
    pub name: String,
    
    /// Creation timestamp
    #[serde(default)]
    pub created_at: String,
    
    /// Expiration timestamp
    #[serde(default)]
    pub expires_at: String,
    
    /// Whether the snapshot is stateful
    #[serde(default)]
    pub stateful: bool,
    
    /// Size of the snapshot in bytes
    #[serde(default)]
    pub size: i64,
}

/// Request to create an instance snapshot
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceSnapshotsPost {
    /// Snapshot name
    pub name: String,
    
    /// Whether to include instance state (memory)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stateful: Option<bool>,
    
    /// Expiration timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl InstanceSnapshotsPost {
    /// Create a new snapshot request
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            stateful: None,
            expires_at: None,
        }
    }
    
    /// Set as stateful snapshot (includes memory)
    pub fn stateful(mut self, stateful: bool) -> Self {
        self.stateful = Some(stateful);
        self
    }
    
    /// Set expiration timestamp
    pub fn expires_at(mut self, expires: impl Into<String>) -> Self {
        self.expires_at = Some(expires.into());
        self
    }
}

/// Request to rename a snapshot
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceSnapshotPost {
    /// New snapshot name
    pub name: String,
}

impl InstanceSnapshotPost {
    /// Create a rename request
    pub fn rename(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
        }
    }
}
