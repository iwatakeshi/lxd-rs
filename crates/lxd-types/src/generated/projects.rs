//!//! Projects types for LXD API
#![allow(clippy::derive_partial_eq_without_eq)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[allow(unused_imports)]
use super::*;
///Project represents a LXD project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///Project configuration map (refer to doc/projects.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The project name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///List of URLs of objects using this project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_by: Option<Vec<String>>,
}
///ProjectPost represents the fields required to rename a LXD project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectPost {
    ///The new name for the project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
///ProjectPut represents the modifiable fields of a LXD project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectPut {
    ///Project configuration map (refer to doc/projects.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
///ProjectState represents the current running state of a LXD project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectState {
    ///Allocated and used resources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<BTreeMap<String, ProjectStateResource>>,
}
///ProjectStateResource represents the state of a particular resource in a LXD project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectStateResource {
    ///Limit for the resource (-1 if none)
    #[serde(rename = "Limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    ///Current usage for the resource
    #[serde(rename = "Usage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<i64>,
}
///ProjectsPost represents the fields of a new LXD project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectsPost {
    ///Project configuration map (refer to doc/projects.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The name of the new project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Add a network device connected to the specified network to the default profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    ///Add a root disk device using the specified storage pool to the default profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
}
