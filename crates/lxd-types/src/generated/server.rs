//!//! Server types for LXD API
#![allow(clippy::derive_partial_eq_without_eq)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[allow(unused_imports)]
use super::*;
///Server represents a LXD server
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Server {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///List of supported API extensions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_extensions: Option<Vec<String>>,
    ///Support status of the current API (one of "devel", "stable" or "deprecated")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_status: Option<String>,
    ///API version number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    ///Whether the client is trusted (one of "trusted" or "untrusted")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    ///List of supported authentication methods
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_methods: Option<Vec<String>>,
    ///The current user login method as seen by LXD
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_user_method: Option<String>,
    ///The current user username as seen by LXD
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_user_name: Option<String>,
    ///Whether the requester sent a client certificate with the request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_certificate: Option<bool>,
    ///Server configuration map (refer to doc/server.md) The available fields for public endpoint (before authentication) are limited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<ServerEnvironment>,
    ///Whether the server is public-only (only public endpoints are implemented)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerEnvironment {
    ///List of addresses the server is listening on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    ///List of architectures supported by the server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<String>>,
    ///Range of supported backup metadata versions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_metadata_version_range: Option<Vec<i64>>,
    ///Server certificate as PEM encoded X509
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    ///Server certificate fingerprint as SHA256
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate_fingerprint: Option<String>,
    ///List of supported instance drivers (separate by " | ")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    ///List of supported instance driver versions (separate by " | ")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver_version: Option<String>,
    ///Current firewall driver
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firewall: Option<String>,
    ///List of supported instance types
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    ///OS kernel name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel: Option<String>,
    ///OS kernel architecture
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_architecture: Option<String>,
    ///Map of kernel features that were tested on startup
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_features: Option<BTreeMap<String, String>>,
    ///Kernel version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    ///Map of LXC features that were tested on startup
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lxc_features: Option<BTreeMap<String, String>>,
    ///Name of the operating system (Linux distribution)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    ///Version of the operating system (Linux distribution)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    ///Current project name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///Server implementation name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    ///Whether the server is part of a cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_clustered: Option<bool>,
    /**Mode that the event distribution subsystem is operating in on this server.
Either "full-mesh", "hub-server" or "hub-client".*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_event_mode: Option<String>,
    ///Whether the version is an LTS release
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_lts: Option<bool>,
    ///Server hostname
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    ///PID of the LXD process
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_pid: Option<i64>,
    ///Server version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_version: Option<String>,
    ///List of active storage drivers (separate by " | ")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    ///List of supported storage drivers
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_supported_drivers: Option<Vec<ServerStorageDriverInfo>>,
    ///List of active storage driver versions (separate by " | ")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_version: Option<String>,
}
///ServerPut represents the modifiable fields of a LXD server configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerPut {
    ///Server configuration map (refer to doc/server.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, serde_json::Value>>,
}
///ServerStorageDriverInfo represents the read-only info about a storage driver
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerStorageDriverInfo {
    ///Name of the driver
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Whether the driver has remote volumes
    #[serde(rename = "Remote")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote: Option<bool>,
    ///Version of the driver
    #[serde(rename = "Version")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
///ServerUntrusted represents a LXD server for an untrusted client
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerUntrusted {
    ///List of supported API extensions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_extensions: Option<Vec<String>>,
    ///Support status of the current API (one of "devel", "stable" or "deprecated")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_status: Option<String>,
    ///API version number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    ///Whether the client is trusted (one of "trusted" or "untrusted")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    ///List of supported authentication methods
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_methods: Option<Vec<String>>,
    ///Whether the requester sent a client certificate with the request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_certificate: Option<bool>,
    ///Server configuration map (refer to doc/server.md) The available fields for public endpoint (before authentication) are limited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, serde_json::Value>>,
    ///Whether the server is public-only (only public endpoints are implemented)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
}
