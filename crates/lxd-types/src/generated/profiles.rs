//!//! Profiles types for LXD API
#![allow(clippy::derive_partial_eq_without_eq)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
///Profile represents a LXD profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///Instance configuration map (refer to doc/instances.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///List of devices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
    ///The profile name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Project name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///List of URLs of objects using this profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_by: Option<Vec<String>>,
}
///ProfilePost represents the fields required to rename a LXD profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfilePost {
    ///The new name for the profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
///ProfilePut represents the modifiable fields of a LXD profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfilePut {
    ///Instance configuration map (refer to doc/instances.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///List of devices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
}
///ProfilesPost represents the fields of a new LXD profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfilesPost {
    ///Instance configuration map (refer to doc/instances.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///List of devices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
    ///The name of the new profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
