//!//! Common types for LXD API
#![allow(clippy::derive_partial_eq_without_eq)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[allow(unused_imports)]
use super::*;
///Event represents an event entry (over websocket)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    ///Originating cluster member
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    ///JSON encoded metadata (see EventLogging, EventLifecycle or Operation)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
    ///Project the event belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///Time at which the event was sent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    ///Event type (one of operation, logging or lifecycle)
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentitiesBearerPost {
    ///Groups is the list of groups for which the identity is a member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    ///Name associated with the identity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Type of identity
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentitiesTLSPost {
    ///The PEM encoded x509 certificate of the identity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    ///Groups is the list of groups for which the identity is a member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    ///Name associated with the identity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Whether to create a certificate add token
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<bool>,
    ///Trust token (used to add an untrusted client)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust_token: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitClusterPreseed {
    ///The address of the cluster you wish to join
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_address: Option<String>,
    ///The expected certificate (X509 PEM encoded) for the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_certificate: Option<String>,
    ///The path to the cluster certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_certificate_path: Option<String>,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitLocalPreseed {
    ///Server configuration map (refer to doc/server.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, serde_json::Value>>,
    ///Networks by project to add to LXD
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<InitNetworksProjectPost>>,
    ///Profiles to add to LXD
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<ProfilesPost>>,
    ///Projects to add to LXD
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectsPost>>,
    ///Storage Pools to add to LXD
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_pools: Option<Vec<StoragePoolsPost>>,
    ///Storage Volumes to add to LXD
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_volumes: Option<Vec<InitStorageVolumesProjectPost>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitNetworksProjectPost {
    ///Project in which the network will reside
    #[serde(rename = "Project")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///Network configuration map (refer to doc/networks.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The name of the new network
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The network type (refer to doc/networks.md)
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitPreseed {
    #[serde(rename = "Node")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<InitLocalPreseed>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<InitClusterPreseed>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitStorageVolumesProjectPost {
    ///Storage pool in which the volume will reside
    #[serde(rename = "Pool")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    ///Project in which the volume will reside
    #[serde(rename = "Project")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///Storage volume configuration map (refer to doc/storage.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Volume content type (filesystem or block)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    ///Description of the storage volume
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Volume name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Name of a snapshot to restore
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<StorageVolumeSource>,
    ///Volume type (container, custom, image or virtual-machine)
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetadataConfiguration {
    ///Configs contains all server configuration metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configs: Option<
        BTreeMap<String, BTreeMap<String, MetadataConfigurationConfigKeys>>,
    >,
    /**Entities contains all authorization related metadata.

API extension: metadata_configuration_entity_types*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<BTreeMap<String, MetadataConfigurationEntity>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetadataConfigurationConfigKey {
    ///Condition describes conditions under which the configuration key can be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    ///DefaultDescription contains a description of the configuration key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaultdesc: Option<String>,
    ///LongDescription contains a long-form description of the configuration key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longdesc: Option<String>,
    ///Managed describes whether the configuration key is managed by LXD.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<String>,
    ///Required describes conditions under which the configuration key is required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<String>,
    ///Scope describes the cluster member specificity of a configuration key. Options marked with a `global` scope are applied to all cluster members. Options marked with a `local` scope are set on a per-member basis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    ///ShortDescription contains a short-form description of the configuration key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shortdesc: Option<String>,
    ///Type describes the type of the key.
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetadataConfigurationConfigKeys {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<BTreeMap<String, MetadataConfigurationConfigKey>>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetadataConfigurationEntity {
    ///Entitlements contains a list of entitlements that apply to a specific entity type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<MetadataConfigurationEntityEntitlement>>,
    ///ProjectSpecific indicates whether the entity is project specific.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_specific: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetadataConfigurationEntityEntitlement {
    ///Description describes the entitlement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name contains the name of the entitlement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OIDCSession {
    ///CreatedAt is when the session was started.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    ///Email is the email of the user that holds the session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///ExpiresAt is when the session will expire.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    ///IP is the IP address of the user that holds the session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    ///UserAgent is the UserAgent of the user that holds the session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    ///Username is the name of the user that holds the session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    ///UUID is the session UUID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
pub type StatusCode = i64;
///that is, entities that can have access entitlements granted to the requesting user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithEntitlements {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
}
