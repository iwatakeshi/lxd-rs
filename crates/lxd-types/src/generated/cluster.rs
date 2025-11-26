//!//! Cluster types for LXD API
#![allow(clippy::derive_partial_eq_without_eq)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    ///Whether clustering is enabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    ///List of member configuration keys (used during join)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_config: Option<Vec<ClusterMemberConfigKey>>,
    ///Name of the cluster member answering the request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
}
///ClusterCertificatePut represents the certificate and key pair for all members in a LXD Cluster
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterCertificatePut {
    ///The new certificate (X509 PEM encoded) for the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_certificate: Option<String>,
    ///The new certificate key (X509 PEM encoded) for the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_certificate_key: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterGroup {
    ///The description of the cluster group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///List of members in this group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    ///The new name of the cluster group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**UsedBy is a list or LXD entity URLs that reference the cluster group.

    API extension: clustering_groups_used_by*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_by: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterGroupPost {
    ///The new name of the cluster group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterGroupPut {
    ///The description of the cluster group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///List of members in this group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterGroupsPost {
    ///The description of the cluster group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///List of members in this group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    ///The new name of the cluster group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterMember {
    ///The primary architecture of the cluster member
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    ///Additional configuration information
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Whether the cluster member is a database server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<bool>,
    ///Cluster member description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name of the failure domain for this cluster member
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_domain: Option<String>,
    ///List of cluster groups this member belongs to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    ///Additional status information
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    ///List of roles held by this cluster member
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    ///Name of the cluster member
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    ///Current status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///URL at which the cluster member can be reached
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
/**The Value field is empty when getting clustering information with GET
1.0/cluster, and should be filled by the joining node when performing a PUT
1.0/cluster join request.*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterMemberConfigKey {
    ///A human friendly description key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The kind of configuration key (network, storage-pool, ...)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    ///The name of the key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    ///The name of the object requiring this key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The value on the answering cluster member
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterMemberJoinToken {
    ///The addresses of existing online cluster members
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    ///The token's expiry date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    ///The fingerprint of the network certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    ///The random join secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    ///The name of the new cluster member
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterMemberPost {
    ///The new name of the cluster member
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
}
///ClusterMemberPut represents the modifiable fields of a LXD cluster member
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterMemberPut {
    ///Additional configuration information
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Cluster member description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name of the failure domain for this cluster member
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_domain: Option<String>,
    ///List of cluster groups this member belongs to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    ///List of roles held by this cluster member
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterMemberState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_pools: Option<BTreeMap<String, StoragePoolState>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sysinfo: Option<ClusterMemberSysInfo>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterMemberStatePost {
    ///The action to be performed. Valid actions are "evacuate" and "restore".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /**Override the configured evacuation mode.
    Valid modes for the "evacuate" action are "stop", "migrate", and "live-migrate".
    Valid modes for the "restore" action are "skip".*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterMemberSysInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buffered_ram: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_ram: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_swap: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_averages: Option<Vec<f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_cpus: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_ram: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_ram: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_swap: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uptime: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterMembersPost {
    ///The name of the new cluster member
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
}
/**ClusterPut represents the fields required to bootstrap or join a LXD
cluster.*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusterPut {
    ///The address of the cluster you wish to join
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_address: Option<String>,
    ///The expected certificate (X509 PEM encoded) for the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_certificate: Option<String>,
    ///The cluster join token for the cluster you're trying to join
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_token: Option<String>,
    ///Whether clustering is enabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    ///List of member configuration keys (used during join)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_config: Option<Vec<ClusterMemberConfigKey>>,
    ///The local address to use for cluster communication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_address: Option<String>,
    ///Name of the cluster member answering the request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
}
