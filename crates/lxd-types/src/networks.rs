//! Network types for LXD API

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Network represents an LXD network
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Network {
    /// Network name
    pub name: String,
    
    /// Network description
    #[serde(default)]
    pub description: String,
    
    /// Network type (bridge, macvlan, physical, etc.)
    #[serde(rename = "type", default)]
    pub network_type: String,
    
    /// Network configuration
    #[serde(default)]
    pub config: BTreeMap<String, String>,
    
    /// Whether the network is managed by LXD
    #[serde(default)]
    pub managed: bool,
    
    /// Network status
    #[serde(default)]
    pub status: String,
    
    /// Resources using this network
    #[serde(default)]
    pub used_by: Vec<String>,
    
    /// Locations (for clusters)
    #[serde(default)]
    pub locations: Vec<String>,
}

/// Request to create a new network
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworksPost {
    /// Network name
    pub name: String,
    
    /// Network description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Network type
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    
    /// Network configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
}

impl NetworksPost {
    /// Create a new network request
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            network_type: None,
            config: None,
        }
    }
    
    /// Set the network type
    pub fn with_type(mut self, network_type: impl Into<String>) -> Self {
        self.network_type = Some(network_type.into());
        self
    }
    
    /// Create a bridge network
    pub fn bridge(name: impl Into<String>) -> Self {
        Self::new(name).with_type("bridge")
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

/// Request to update a network
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkPut {
    /// Network description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Network configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
}
