//! Instance (container/VM) types for LXD API

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Instance type (container or virtual machine)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum InstanceType {
    #[default]
    Container,
    #[serde(rename = "virtual-machine")]
    VirtualMachine,
}


/// Instance status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstanceStatus {
    Running,
    Stopped,
    Frozen,
    Error,
    Starting,
    Stopping,
}

/// Instance represents an LXD instance (container or VM)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instance {
    /// Instance name
    pub name: String,
    
    /// Instance description
    #[serde(default)]
    pub description: String,
    
    /// Instance status
    pub status: String,
    
    /// Instance status code
    pub status_code: i64,
    
    /// Instance type
    #[serde(rename = "type", default)]
    pub instance_type: InstanceType,
    
    /// Architecture name
    #[serde(default)]
    pub architecture: String,
    
    /// Whether the instance is ephemeral
    #[serde(default)]
    pub ephemeral: bool,
    
    /// Whether the instance is stateful
    #[serde(default)]
    pub stateful: bool,
    
    /// Instance configuration
    #[serde(default)]
    pub config: BTreeMap<String, String>,
    
    /// Instance devices
    #[serde(default)]
    pub devices: BTreeMap<String, BTreeMap<String, String>>,
    
    /// Expanded configuration (including profile)
    #[serde(default)]
    pub expanded_config: BTreeMap<String, String>,
    
    /// Expanded devices (including profile)
    #[serde(default)]
    pub expanded_devices: BTreeMap<String, BTreeMap<String, String>>,
    
    /// Profiles applied to this instance
    #[serde(default)]
    pub profiles: Vec<String>,
    
    /// Creation timestamp
    #[serde(default)]
    pub created_at: String,
    
    /// Last used timestamp
    #[serde(default)]
    pub last_used_at: String,
    
    /// Instance location (for clusters)
    #[serde(default)]
    pub location: String,
    
    /// Project name
    #[serde(default)]
    pub project: String,
}

/// Request to create a new instance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstancesPost {
    /// Instance name
    pub name: String,
    
    /// Instance description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Instance type
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<InstanceType>,
    
    /// Instance architecture
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    
    /// Whether the instance is ephemeral
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,
    
    /// Instance configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    
    /// Instance devices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
    
    /// Profiles to apply
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    
    /// Instance source
    pub source: InstanceSource,
}

impl InstancesPost {
    /// Create a new instance creation request
    pub fn new(name: impl Into<String>, source: InstanceSource) -> Self {
        Self {
            name: name.into(),
            description: None,
            instance_type: None,
            architecture: None,
            ephemeral: None,
            config: None,
            devices: None,
            profiles: None,
            source,
        }
    }
    
    /// Set the instance type
    pub fn with_type(mut self, instance_type: InstanceType) -> Self {
        self.instance_type = Some(instance_type);
        self
    }
    
    /// Set ephemeral mode
    pub fn ephemeral(mut self, ephemeral: bool) -> Self {
        self.ephemeral = Some(ephemeral);
        self
    }
    
    /// Set profiles
    pub fn with_profiles(mut self, profiles: Vec<String>) -> Self {
        self.profiles = Some(profiles);
        self
    }
}

/// Instance source for creation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceSource {
    /// Source type
    #[serde(rename = "type")]
    pub source_type: String,
    
    /// Image alias or fingerprint (for "image" type)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    
    /// Image fingerprint (for "image" type)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    
    /// Image properties (for "image" type)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, String>>,
    
    /// Image server (for remote images)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    
    /// Protocol for remote images
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    
    /// Source container name (for "copy" type)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl InstanceSource {
    /// Create source from an image alias
    pub fn from_image(alias: impl Into<String>) -> Self {
        Self {
            source_type: "image".to_string(),
            alias: Some(alias.into()),
            fingerprint: None,
            properties: None,
            server: None,
            protocol: None,
            source: None,
        }
    }
    
    /// Create source from a remote image
    pub fn from_remote_image(alias: impl Into<String>, server: impl Into<String>) -> Self {
        Self {
            source_type: "image".to_string(),
            alias: Some(alias.into()),
            fingerprint: None,
            properties: None,
            server: Some(server.into()),
            protocol: Some("simplestreams".to_string()),
            source: None,
        }
    }
    
    /// Create an empty source (for empty instances)
    pub fn none() -> Self {
        Self {
            source_type: "none".to_string(),
            alias: None,
            fingerprint: None,
            properties: None,
            server: None,
            protocol: None,
            source: None,
        }
    }
    
    /// Create source from copying another instance
    pub fn from_copy(source_name: impl Into<String>) -> Self {
        Self {
            source_type: "copy".to_string(),
            alias: None,
            fingerprint: None,
            properties: None,
            server: None,
            protocol: None,
            source: Some(source_name.into()),
        }
    }
}

/// Request to update an instance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InstancePut {
    /// Instance description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Instance architecture
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    
    /// Instance configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    
    /// Instance devices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
    
    /// Whether the instance is ephemeral
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,
    
    /// Profiles to apply
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
}

/// Instance state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceState {
    /// Current status
    pub status: String,
    
    /// Status code
    pub status_code: i64,
    
    /// CPU usage
    #[serde(default)]
    pub cpu: InstanceStateCpu,
    
    /// Disk usage
    #[serde(default)]
    pub disk: BTreeMap<String, InstanceStateDisk>,
    
    /// Memory usage
    #[serde(default)]
    pub memory: InstanceStateMemory,
    
    /// Network usage
    #[serde(default)]
    pub network: BTreeMap<String, InstanceStateNetwork>,
    
    /// PID of init process
    pub pid: i64,
    
    /// Number of processes
    pub processes: i64,
}

/// CPU state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceStateCpu {
    /// CPU usage in nanoseconds
    #[serde(default)]
    pub usage: i64,
}

/// Disk state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceStateDisk {
    /// Disk usage in bytes
    #[serde(default)]
    pub usage: i64,
    
    /// Total disk space in bytes
    #[serde(default)]
    pub total: i64,
}

/// Memory state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceStateMemory {
    /// Memory usage in bytes
    #[serde(default)]
    pub usage: i64,
    
    /// Peak memory usage in bytes
    #[serde(default)]
    pub usage_peak: i64,
    
    /// Total memory in bytes
    #[serde(default)]
    pub total: i64,
    
    /// Swap usage in bytes
    #[serde(default)]
    pub swap_usage: i64,
    
    /// Peak swap usage in bytes
    #[serde(default)]
    pub swap_usage_peak: i64,
}

/// Network state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceStateNetwork {
    /// Network addresses
    #[serde(default)]
    pub addresses: Vec<InstanceStateNetworkAddress>,
    
    /// Packet counters
    #[serde(default)]
    pub counters: InstanceStateNetworkCounters,
    
    /// Hardware address (MAC)
    #[serde(default)]
    pub hwaddr: String,
    
    /// Host name
    #[serde(default)]
    pub host_name: String,
    
    /// MTU
    #[serde(default)]
    pub mtu: i64,
    
    /// State
    #[serde(default)]
    pub state: String,
    
    /// Type
    #[serde(rename = "type", default)]
    pub network_type: String,
}

/// Network address
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceStateNetworkAddress {
    /// Address family
    pub family: String,
    
    /// IP address
    pub address: String,
    
    /// Network mask
    pub netmask: String,
    
    /// Scope
    pub scope: String,
}

/// Network counters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceStateNetworkCounters {
    #[serde(default)]
    pub bytes_received: i64,
    #[serde(default)]
    pub bytes_sent: i64,
    #[serde(default)]
    pub packets_received: i64,
    #[serde(default)]
    pub packets_sent: i64,
    #[serde(default)]
    pub errors_received: i64,
    #[serde(default)]
    pub errors_sent: i64,
    #[serde(default)]
    pub packets_dropped_outbound: i64,
    #[serde(default)]
    pub packets_dropped_inbound: i64,
}

/// Request to change instance state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceStatePut {
    /// Desired action
    pub action: InstanceAction,
    
    /// Timeout in seconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    
    /// Force the action
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    
    /// Whether the instance is stateful
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stateful: Option<bool>,
}

/// Instance action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstanceAction {
    Start,
    Stop,
    Restart,
    Freeze,
    Unfreeze,
}
