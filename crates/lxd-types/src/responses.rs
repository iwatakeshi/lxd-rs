//! LXD API response types

use serde::{Deserialize, Serialize};

/// Generic LXD API response wrapper
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response<T> {
    /// Response type (sync or async)
    #[serde(rename = "type")]
    pub response_type: ResponseType,

    /// Status description
    pub status: String,

    /// Status code
    pub status_code: i64,

    /// Operation URL (for async responses)
    #[serde(default)]
    pub operation: String,

    /// Error message
    #[serde(default)]
    pub error: String,

    /// Error code
    #[serde(default)]
    pub error_code: i64,

    /// Response metadata
    pub metadata: T,
}

/// Response type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseType {
    /// Synchronous response with immediate result
    Sync,
    /// Asynchronous response with operation URL
    Async,
    /// Error response
    Error,
}

/// Empty metadata for responses without data
pub type EmptyResponse = Response<()>;

/// List response (URLs)
pub type ListResponse = Response<Vec<String>>;

/// Sync response with arbitrary JSON
pub type JsonResponse = Response<serde_json::Value>;

impl<T> Response<T> {
    /// Check if the response indicates success
    pub fn is_success(&self) -> bool {
        self.status_code >= 100 && self.status_code < 400
    }

    /// Check if this is an async operation
    pub fn is_async(&self) -> bool {
        self.response_type == ResponseType::Async
    }

    /// Check if this is an error response
    pub fn is_error(&self) -> bool {
        self.response_type == ResponseType::Error || self.status_code >= 400
    }

    /// Get the operation ID from the operation URL
    pub fn operation_id(&self) -> Option<&str> {
        if self.operation.is_empty() {
            return None;
        }
        self.operation.rsplit('/').next()
    }
}

// Server types are only defined here when NOT using generated feature
// When generated feature is enabled, use lxd_types::server::Server instead
#[cfg(not(feature = "generated"))]
/// Server information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Server {
    /// API extensions supported
    #[serde(default)]
    pub api_extensions: Vec<String>,

    /// API status
    #[serde(default)]
    pub api_status: String,

    /// API version
    #[serde(default)]
    pub api_version: String,

    /// Authentication methods
    #[serde(default)]
    pub auth: String,

    /// Authentication user name
    #[serde(default)]
    pub auth_user_name: String,

    /// Authentication user method
    #[serde(default)]
    pub auth_user_method: String,

    /// Whether the server is public (unauthenticated)
    #[serde(default)]
    pub public: bool,

    /// Server configuration
    #[serde(default)]
    pub config: serde_json::Value,

    /// Server environment
    #[serde(default)]
    pub environment: ServerEnvironment,
}

#[cfg(not(feature = "generated"))]
/// Server environment information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEnvironment {
    /// Architecture
    #[serde(default)]
    pub architecture: String,

    /// Architectures supported
    #[serde(default)]
    pub architectures: Vec<String>,

    /// Certificate
    #[serde(default)]
    pub certificate: String,

    /// Certificate fingerprint
    #[serde(default)]
    pub certificate_fingerprint: String,

    /// Driver
    #[serde(default)]
    pub driver: String,

    /// Driver version
    #[serde(default)]
    pub driver_version: String,

    /// Firewall driver
    #[serde(default)]
    pub firewall: String,

    /// Kernel
    #[serde(default)]
    pub kernel: String,

    /// Kernel architecture
    #[serde(default)]
    pub kernel_architecture: String,

    /// Kernel features
    #[serde(default)]
    pub kernel_features: serde_json::Value,

    /// Kernel version
    #[serde(default)]
    pub kernel_version: String,

    /// LXC features
    #[serde(default)]
    pub lxc_features: serde_json::Value,

    /// Operating system name
    #[serde(default)]
    pub os_name: String,

    /// Operating system version
    #[serde(default)]
    pub os_version: String,

    /// Project
    #[serde(default)]
    pub project: String,

    /// Server
    #[serde(default)]
    pub server: String,

    /// Server clustered
    #[serde(default)]
    pub server_clustered: bool,

    /// Server event mode
    #[serde(default)]
    pub server_event_mode: String,

    /// Server name
    #[serde(default)]
    pub server_name: String,

    /// Server PID
    #[serde(default)]
    pub server_pid: i64,

    /// Server version
    #[serde(default)]
    pub server_version: String,

    /// Storage
    #[serde(default)]
    pub storage: String,

    /// Storage supported drivers
    #[serde(default)]
    pub storage_supported_drivers: Vec<serde_json::Value>,

    /// Storage version
    #[serde(default)]
    pub storage_version: String,
}
