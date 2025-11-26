//! Project types for LXD API

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Project represents an LXD project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    /// Project name
    pub name: String,
    
    /// Project description
    #[serde(default)]
    pub description: String,
    
    /// Project configuration
    #[serde(default)]
    pub config: BTreeMap<String, String>,
    
    /// Resources used by the project
    #[serde(default)]
    pub used_by: Vec<String>,
}

/// Request to create a new project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectsPost {
    /// Project name
    pub name: String,
    
    /// Project description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Project configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
}

impl ProjectsPost {
    /// Create a new project request
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            config: None,
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
}

/// Request to update a project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectPut {
    /// Project description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Project configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
}
