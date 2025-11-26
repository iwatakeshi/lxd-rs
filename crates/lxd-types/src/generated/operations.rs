//!//! Operations types for LXD API
#![allow(clippy::derive_partial_eq_without_eq)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
///Operation represents a LXD background operation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    ///Type of operation (task, token or websocket)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    ///Operation creation time
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    ///Description of the operation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Operation error mesage
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    ///UUID of the operation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///What cluster member this record was found on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    ///Whether the operation can be canceled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub may_cancel: Option<bool>,
    ///Operation specific metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requestor: Option<OperationRequestor>,
    ///Affected resourcs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<BTreeMap<String, Vec<String>>>,
    ///Status name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<StatusCode>,
    ///Operation last change
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
///API extension: operation_requestor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationRequestor {
    ///Address is the origin address of the request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    ///Protocol represents the method used to authenticate the requestor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    ///Username is the username of the requestor. This is the identifier of the identity, or the username if using the unix socket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
