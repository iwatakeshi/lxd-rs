//!//! Storage types for LXD API
#![allow(clippy::derive_partial_eq_without_eq)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[allow(unused_imports)]
use super::*;
///StorageBucket represents the fields of a LXD storage pool bucket
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageBucket {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///Storage bucket configuration map
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the storage bucket
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///What cluster member this record was found on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    ///Bucket name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Project name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///Bucket S3 URL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3_url: Option<String>,
}
///StorageBucketKey represents the fields of a LXD storage pool bucket key
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageBucketKey {
    ///Access key
    #[serde(rename = "access-key")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    ///Description of the storage bucket key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Key name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Whether the key can perform write actions or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    ///Secret key
    #[serde(rename = "secret-key")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
}
///StorageBucketKeyPut represents the modifiable fields of a LXD storage pool bucket key
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageBucketKeyPut {
    ///Access key
    #[serde(rename = "access-key")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    ///Description of the storage bucket key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Whether the key can perform write actions or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    ///Secret key
    #[serde(rename = "secret-key")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
}
///StorageBucketKeysPost represents the fields of a new LXD storage pool bucket key
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageBucketKeysPost {
    ///Access key
    #[serde(rename = "access-key")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    ///Description of the storage bucket key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Key name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Whether the key can perform write actions or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    ///Secret key
    #[serde(rename = "secret-key")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
}
///StorageBucketPut represents the modifiable fields of a LXD storage pool bucket
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageBucketPut {
    ///Storage bucket configuration map
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the storage bucket
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
///StorageBucketsPost represents the fields of a new LXD storage pool bucket
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageBucketsPost {
    ///Storage bucket configuration map
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the storage bucket
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Bucket name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoragePool {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///Storage pool configuration map (refer to doc/storage.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the storage pool
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Storage pool driver (btrfs, ceph, cephfs, dir, lvm or zfs)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    ///Cluster members on which the storage pool has been defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<String>>,
    ///Storage pool name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Pool status (Pending, Created, Errored or Unknown)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///List of URLs of objects using this storage pool
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_by: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolPut {
    ///Storage pool configuration map (refer to doc/storage.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the storage pool
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inodes: Option<ResourcesStoragePoolInodes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space: Option<ResourcesStoragePoolSpace>,
}
///StoragePoolVolumeBackup represents a LXD volume backup
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolVolumeBackup {
    ///When the backup was created
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    ///When the backup expires (gets auto-deleted)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    ///Backup name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Whether to use a pool-optimized binary format (instead of plain tarball)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optimized_storage: Option<bool>,
    ///Whether to ignore snapshots
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_only: Option<bool>,
}
///StoragePoolVolumeBackupPost represents the fields available for the renaming of a volume backup
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolVolumeBackupPost {
    ///New backup name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
///StoragePoolVolumeBackupsPost represents the fields available for a new LXD volume backup
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolVolumeBackupsPost {
    ///What compression algorithm to use
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression_algorithm: Option<String>,
    ///When the backup expires (gets auto-deleted)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    ///Backup name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Whether to use a pool-optimized binary format (instead of plain tarball)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optimized_storage: Option<bool>,
    ///What backup format version to use
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    ///Whether to ignore snapshots
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_only: Option<bool>,
}
///StoragePoolsPost represents the fields of a new LXD storage pool
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolsPost {
    ///Storage pool configuration map (refer to doc/storage.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the storage pool
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Storage pool driver (btrfs, ceph, cephfs, dir, lvm or zfs)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    ///Storage pool name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageVolume {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///Storage volume configuration map (refer to doc/storage.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Volume content type (filesystem or block)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    ///Volume creation timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    ///Description of the storage volume
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///What cluster member this record was found on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    ///Volume name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Name of the pool the volume is using
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    ///Project containing the volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///Volume type
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    ///List of URLs of objects using this storage volume
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_by: Option<Vec<String>>,
}
///StorageVolumePost represents the fields required to rename a LXD storage pool volume
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageVolumePost {
    ///Initiate volume migration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration: Option<bool>,
    ///New volume name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///New storage pool
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    ///New project name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<StorageVolumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<StorageVolumePostTarget>,
    ///Whether snapshots should be discarded (migration only)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_only: Option<bool>,
}
///StorageVolumePostTarget represents the migration target host and operation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageVolumePostTarget {
    ///The certificate of the migration target
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    ///Remote operation URL (for migration)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    ///Migration websockets credentials
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<BTreeMap<String, String>>,
}
///StorageVolumePut represents the modifiable fields of a LXD storage volume
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageVolumePut {
    ///Storage volume configuration map (refer to doc/storage.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the storage volume
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name of a snapshot to restore
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore: Option<String>,
}
///StorageVolumeSnapshot represents a LXD storage volume snapshot
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageVolumeSnapshot {
    ///Storage volume configuration map (refer to doc/storage.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///The content type (filesystem or block)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    ///Volume snapshot creation timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    ///Description of the storage volume
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///When the snapshot expires (gets auto-deleted)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    ///Snapshot name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
///StorageVolumeSnapshotPost represents the fields required to rename/move a LXD storage volume snapshot
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageVolumeSnapshotPost {
    ///Initiate volume snapshot migration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration: Option<bool>,
    ///New snapshot name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<StorageVolumePostTarget>,
}
///StorageVolumeSnapshotPut represents the modifiable fields of a LXD storage volume
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageVolumeSnapshotPut {
    ///Description of the storage volume
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///When the snapshot expires (gets auto-deleted)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}
///StorageVolumeSnapshotsPost represents the fields available for a new LXD storage volume snapshot
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageVolumeSnapshotsPost {
    ///Description of the storage volume snapshot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///When the snapshot expires (gets auto-deleted)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    ///Snapshot name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
///StorageVolumeSource represents the creation source for a new storage volume
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageVolumeSource {
    ///Certificate (for migration)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    ///What cluster member this record was found on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    ///Whether to use pull or push mode (for migration)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    ///Source volume name (for copy)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Remote operation URL (for migration)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    ///Source storage pool (for copy)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    ///Source project name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///Whether existing destination volume should be refreshed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh: Option<bool>,
    ///Map of migration websockets (for migration)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<BTreeMap<String, String>>,
    ///Source type (copy or migration)
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    ///Whether snapshots should be discarded (for migration)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_only: Option<bool>,
}
///StorageVolumeState represents the live state of the volume
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageVolumeState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<StorageVolumeStateUsage>,
}
///StorageVolumeStateUsage represents the disk usage of a volume
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageVolumeStateUsage {
    ///Storage volume size in bytes. Uses 0 to convey that the volume has access to the entire pool's storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    ///Used space in bytes. Uses 0 to indicate that the storage driver for the pool does not support retrieving volume usage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
}
///StorageVolumesPost represents the fields of a new LXD storage pool volume
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageVolumesPost {
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
