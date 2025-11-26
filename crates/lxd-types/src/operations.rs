//! Operation types for LXD API

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Operation represents a background operation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    /// Operation ID
    pub id: String,

    /// Operation class (task, token, websocket)
    pub class: String,

    /// Operation description
    #[serde(default)]
    pub description: String,

    /// Creation timestamp
    #[serde(default)]
    pub created_at: String,

    /// Update timestamp
    #[serde(default)]
    pub updated_at: String,

    /// Operation status
    pub status: String,

    /// Status code
    pub status_code: i64,

    /// Resources affected by the operation
    #[serde(default)]
    pub resources: BTreeMap<String, Vec<String>>,

    /// Operation metadata
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,

    /// Whether the operation may be cancelled
    #[serde(default)]
    pub may_cancel: bool,

    /// Error message if failed
    #[serde(default)]
    pub err: String,

    /// Operation location (for clusters)
    #[serde(default)]
    pub location: String,
}

/// Operation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationStatus {
    /// Operation is pending
    #[serde(rename = "Pending")]
    Pending,
    /// Operation is running
    #[serde(rename = "Running")]
    Running,
    /// Operation completed successfully
    #[serde(rename = "Success")]
    Success,
    /// Operation failed
    #[serde(rename = "Failure")]
    Failure,
    /// Operation was cancelled
    #[serde(rename = "Cancelled")]
    Cancelled,
}

impl OperationStatus {
    /// Check if the operation is complete
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Success | Self::Failure | Self::Cancelled)
    }

    /// Check if the operation succeeded
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }
}

/// Operation class
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperationClass {
    /// Task operation (normal background operation)
    Task,
    /// Token operation (one-time use)
    Token,
    /// WebSocket operation (interactive)
    Websocket,
}
