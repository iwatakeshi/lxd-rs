//! Profile types for LXD API

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Profile represents an LXD profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    /// Profile name
    pub name: String,
    
    /// Profile description
    #[serde(default)]
    pub description: String,
    
    /// Profile configuration
    #[serde(default)]
    pub config: BTreeMap<String, String>,
    
    /// Profile devices
    #[serde(default)]
    pub devices: BTreeMap<String, BTreeMap<String, String>>,
    
    /// Instances using this profile
    #[serde(default)]
    pub used_by: Vec<String>,
}

/// Request to create a new profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfilesPost {
    /// Profile name
    pub name: String,
    
    /// Profile description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Profile configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    
    /// Profile devices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
}

impl ProfilesPost {
    /// Create a new profile request
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            config: None,
            devices: None,
        }
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
    
    /// Set the devices
    pub fn with_devices(mut self, devices: BTreeMap<String, BTreeMap<String, String>>) -> Self {
        self.devices = Some(devices);
        self
    }
}

/// Request to update a profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfilePut {
    /// Profile description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Profile configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    
    /// Profile devices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
}
