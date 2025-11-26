//!//! Warnings types for LXD API
#![allow(clippy::derive_partial_eq_without_eq)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Warning {
    ///The number of times this warning occurred
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    ///The entity affected by this warning
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_url: Option<String>,
    ///The first time this warning occurred
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_seen_at: Option<String>,
    ///The warning message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message: Option<String>,
    ///The last time this warning occurred
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
    ///What cluster member this warning occurred on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    ///The project the warning occurred in
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///The severity of this warning
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    ///Status of the warning (new, acknowledged, or resolved)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///Type type of warning
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    ///UUID of the warning
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarningPut {
    ///Status of the warning (new, acknowledged, or resolved)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
