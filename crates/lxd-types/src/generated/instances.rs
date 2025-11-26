//!//! Instances types for LXD API
#![allow(clippy::derive_partial_eq_without_eq)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instance {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///Architecture name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    ///Instance configuration (see doc/instances.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Instance creation timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    ///Instance description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Instance devices (see doc/instances.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
    ///Whether the instance is ephemeral (deleted on shutdown)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,
    ///Expanded configuration (all profiles and local config merged)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded_config: Option<BTreeMap<String, String>>,
    ///Expanded devices (all profiles and local devices merged)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded_devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
    ///Last start timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    ///What cluster member this instance is located on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    ///Instance name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///List of profiles applied to the instance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    ///Instance project name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///Whether the instance currently has saved state on disk
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stateful: Option<bool>,
    ///Instance status (see instance_state)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<StatusCode>,
    ///The type of instance (container or virtual-machine)
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceBackup {
    ///Whether to ignore snapshots (deprecated, use instance_only)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_only: Option<bool>,
    ///When the backup was created
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    ///When the backup expires (gets auto-deleted)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    ///Whether to ignore snapshots
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_only: Option<bool>,
    ///Backup name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Whether to use a pool-optimized binary format (instead of plain tarball)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optimized_storage: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceBackupPost {
    ///New backup name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceBackupsPost {
    ///What compression algorithm to use
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression_algorithm: Option<String>,
    ///Whether to ignore snapshots (deprecated, use instance_only)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_only: Option<bool>,
    ///When the backup expires (gets auto-deleted)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    ///Whether to ignore snapshots
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_only: Option<bool>,
    ///Backup name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Whether to use a pool-optimized binary format (instead of plain tarball)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optimized_storage: Option<bool>,
    ///What backup format version to use
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceConsolePost {
    ///Console height in rows (console type only)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    ///Type of console to attach to (console or vga)
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    ///Console width in columns (console type only)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceExecPost {
    ///Command and its arguments
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    ///Current working directory for the command
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    ///Additional environment to pass to the command
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<BTreeMap<String, String>>,
    ///GID of the user to spawn the command as
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,
    ///Terminal height in rows (for interactive)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    ///Whether the command is to be spawned in interactive mode (singled PTY instead of 3 PIPEs)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,
    ///Whether to capture the output for later download (requires non-interactive)
    #[serde(rename = "record-output")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_output: Option<bool>,
    ///UID of the user to spawn the command as
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<i64>,
    ///Whether to wait for all websockets to be connected before spawning the command
    #[serde(rename = "wait-for-websocket")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait_for_websocket: Option<bool>,
    ///Terminal width in characters (for interactive)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceFull {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///Architecture name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    ///List of backups.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<InstanceBackup>>,
    ///Instance configuration (see doc/instances.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Instance creation timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    ///Instance description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Instance devices (see doc/instances.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
    ///Whether the instance is ephemeral (deleted on shutdown)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,
    ///Expanded configuration (all profiles and local config merged)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded_config: Option<BTreeMap<String, String>>,
    ///Expanded devices (all profiles and local devices merged)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded_devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
    ///Last start timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    ///What cluster member this instance is located on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    ///Instance name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///List of profiles applied to the instance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    ///Instance project name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///List of snapshots.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<Vec<InstanceSnapshot>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<InstanceState>,
    ///Whether the instance currently has saved state on disk
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stateful: Option<bool>,
    ///Instance status (see instance_state)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<StatusCode>,
    ///The type of instance (container or virtual-machine)
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstancePost {
    ///Instance configuration file.
    #[serde(rename = "Config")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Instance devices.
    #[serde(rename = "Devices")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
    ///List of profiles applied to the instance.
    #[serde(rename = "Profiles")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    ///AllowInconsistent allow inconsistent copies when migrating.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_inconsistent: Option<bool>,
    ///Whether snapshots should be discarded (migration only, deprecated, use instance_only)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_only: Option<bool>,
    ///Whether snapshots should be discarded (migration only)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_only: Option<bool>,
    ///Whether to perform a live migration (migration only)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live: Option<bool>,
    ///Whether the instance is being migrated to another server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration: Option<bool>,
    ///New name for the instance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Whether the instances's snapshot should receive target instances profile on copy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_snapshot_profiles: Option<bool>,
    ///Target pool for local cross-pool move
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    ///Target project for local cross-project move
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<InstancePostTarget>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstancePostTarget {
    ///The certificate of the migration target
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    ///The operation URL on the remote target
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    ///Migration websockets credentials
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<BTreeMap<String, String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstancePut {
    ///Architecture name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    ///Instance configuration (see doc/instances.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Instance description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Instance devices (see doc/instances.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
    ///Whether the instance is ephemeral (deleted on shutdown)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,
    ///List of profiles applied to the instance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    ///If set, instance will be restored to the provided snapshot name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore: Option<String>,
    ///Which disk volumes to restore from an instance snapshot. Possible values are "root" or "all-exclusive".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_disk_volumes_mode: Option<String>,
    ///Whether the instance currently has saved state on disk
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stateful: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceRebuildPost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<InstanceSource>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceSnapshot {
    ///Architecture name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    ///Instance configuration (see doc/instances.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Instance creation timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    ///Instance devices (see doc/instances.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
    ///Whether the instance is ephemeral (deleted on shutdown)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,
    ///Expanded configuration (all profiles and local config merged)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded_config: Option<BTreeMap<String, String>>,
    ///Expanded devices (all profiles and local devices merged)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded_devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
    ///When the snapshot expires (gets auto-deleted)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    ///Last start timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    ///Snapshot name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///List of profiles applied to the instance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    ///Size of the snapshot in bytes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    ///Whether the instance currently has saved state on disk
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stateful: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceSnapshotPost {
    ///Whether to perform a live migration (requires migration)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live: Option<bool>,
    ///Whether this is a migration request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration: Option<bool>,
    ///New name for the snapshot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<InstancePostTarget>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceSnapshotPut {
    ///When the snapshot expires (gets auto-deleted)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceSnapshotsPost {
    ///Which disk volumes to include in instance snapshot. Possible values are "root" or "all-exclusive".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_volumes_mode: Option<String>,
    ///When the snapshot expires (gets auto-deleted)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    ///Snapshot name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Whether the snapshot should include runtime state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stateful: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceSource {
    ///Image alias name (for image source)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    ///Whether to ignore errors when copying (e.g. for volatile files)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_inconsistent: Option<bool>,
    ///Base image fingerprint (for faster migration)
    #[serde(rename = "base-image")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_image: Option<String>,
    ///Certificate (for remote images or migration)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    ///Whether the copy should skip the snapshots (for copy, deprecated, use instance_only)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_only: Option<bool>,
    ///Optional list of options that are used during image conversion (for conversion).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversion_options: Option<Vec<String>>,
    ///Image fingerprint (for image source)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    ///Whether the copy should skip the snapshots (for copy)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_only: Option<bool>,
    ///Whether this is a live migration (for migration)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live: Option<bool>,
    ///Whether to use pull or push mode (for migration)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    ///Remote operation URL (for migration)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    ///Whether the instances's snapshot should receive target instances profile on copy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_snapshot_profiles: Option<bool>,
    ///Source project name (for copy and local image)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///Image filters (for image source)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, String>>,
    ///Protocol name (for remote image)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    ///Whether this is refreshing an existing instance (for migration and copy)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh: Option<bool>,
    ///Remote server secret (for remote private images)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    ///Map of migration websockets (for migration)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<BTreeMap<String, String>>,
    ///Remote server URL (for remote images)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    ///Existing instance name or snapshot (for copy)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /**Source disk size in bytes used to set the instance's volume size to accommodate the transferred root
disk. This value is ignored if the root disk device has a size explicitly configured (for conversion).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_disk_size: Option<i64>,
    ///Source type
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<InstanceStateCPU>,
    ///Disk usage key/value pairs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk: Option<BTreeMap<String, InstanceStateDisk>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<InstanceStateMemory>,
    ///Network usage key/value pairs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<BTreeMap<String, InstanceStateNetwork>>,
    ///PID of the runtime
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
    ///Number of processes in the instance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processes: Option<i64>,
    ///Current status (Running, Stopped, Frozen or Error)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<StatusCode>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceStateCPU {
    ///CPU usage in nanoseconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceStateDisk {
    ///Total size in bytes. Uses 0 to convey that the instance has access to the entire pool's storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    ///Disk usage in bytes. Uses 0 to indicate that the storage driver for the pool does not support retrieving disk usage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceStateMemory {
    ///SWAP usage in bytes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swap_usage: Option<i64>,
    ///Peak SWAP usage in bytes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swap_usage_peak: Option<i64>,
    ///Total memory size in bytes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    ///Memory usage in bytes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<i64>,
    ///Peak memory usage in bytes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_peak: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceStateNetwork {
    ///List of IP addresses
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<InstanceStateNetworkAddress>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counters: Option<InstanceStateNetworkCounters>,
    ///Name of the interface on the host
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    ///MAC address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hwaddr: Option<String>,
    ///MTU (maximum transmit unit) for the interface
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,
    ///Administrative state of the interface (up/down)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    ///Type of interface (broadcast, loopback, point-to-point, ...)
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
/**InstanceStateNetworkAddress represents a network address as part of the network section of a LXD
instance's state.*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceStateNetworkAddress {
    ///IP address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    ///Network family (inet or inet6)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    ///Network mask
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,
    ///Address scope (local, link or global)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
/**InstanceStateNetworkCounters represents packet counters as part of the network section of a LXD
instance's state.*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceStateNetworkCounters {
    ///Number of bytes received
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes_received: Option<i64>,
    ///Number of bytes sent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes_sent: Option<i64>,
    ///Number of errors received
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors_received: Option<i64>,
    ///Number of errors sent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors_sent: Option<i64>,
    ///Number of inbound packets dropped
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packets_dropped_inbound: Option<i64>,
    ///Number of outbound packets dropped
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packets_dropped_outbound: Option<i64>,
    ///Number of packets received
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packets_received: Option<i64>,
    ///Number of packets sent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packets_sent: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceStatePut {
    ///State change action (start, stop, restart, freeze, unfreeze)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    ///Whether to force the action (for stop and restart)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    ///Whether to store the runtime state (for stop)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stateful: Option<bool>,
    ///How long to wait (in s) before giving up (when force isn't set)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}
pub type InstanceType = String;
///InstanceUEFIVariable represents an EFI variable entry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceUEFIVariable {
    ///UEFI variable attributes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attr: Option<i64>,
    ///UEFI variable data (HEX-encoded)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    ///UEFI variable digest (HEX-encoded)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    ///UEFI variable timestamp (HEX-encoded)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceUEFIVars {
    /**UEFI variables map
Hashmap key format is `<uefi-variable-name>`-`<UUID>`*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<BTreeMap<String, InstanceUEFIVariable>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstancesPost {
    ///Architecture name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    ///Instance configuration (see doc/instances.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Instance description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Instance devices (see doc/instances.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<BTreeMap<String, BTreeMap<String, String>>>,
    ///Whether the instance is ephemeral (deleted on shutdown)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,
    ///Cloud instance type (AWS, GCP, Azure, ...) to emulate with limits
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    ///Instance name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///List of profiles applied to the instance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    ///If set, instance will be restored to the provided snapshot name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore: Option<String>,
    ///Which disk volumes to restore from an instance snapshot. Possible values are "root" or "all-exclusive".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_disk_volumes_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<InstanceSource>,
    ///Whether to start the instance after creation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<bool>,
    ///Whether the instance currently has saved state on disk
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stateful: Option<bool>,
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<InstanceType>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstancesPut {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<InstanceStatePut>,
}
