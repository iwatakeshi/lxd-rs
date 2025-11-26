//!//! Resources types for LXD API
#![allow(clippy::derive_partial_eq_without_eq)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[allow(unused_imports)]
use super::*;
///Resources represents the system resources available for LXD
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<ResourcesCPU>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gpu: Option<ResourcesGPU>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<ResourcesMemory>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<ResourcesNetwork>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pci: Option<ResourcesPCI>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<ResourcesStorage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<ResourcesSystem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usb: Option<ResourcesUSB>,
}
///ResourcesCPU represents the cpu resources available on the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesCPU {
    ///Architecture name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    ///List of CPU sockets
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sockets: Option<Vec<ResourcesCPUSocket>>,
    ///Total number of CPU threads (from all sockets and cores)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}
///ResourcesCPUCache represents a CPU cache
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesCPUCache {
    ///Cache level (usually a number from 1 to 3)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    ///Size of the cache (in bytes)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    ///Type of cache (Data, Instruction, Unified, ...)
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
///ResourcesCPUCore represents a CPU core on the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesCPUCore {
    ///Core identifier within the socket
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub core: Option<i64>,
    ///What die the CPU is a part of (for chiplet designs)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub die: Option<i64>,
    ///Current frequency
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    ///List of threads
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threads: Option<Vec<ResourcesCPUThread>>,
}
///ResourcesCPUSocket represents a CPU socket on the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesCPUSocket {
    ///List of CPU caches
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache: Option<Vec<ResourcesCPUCache>>,
    ///List of CPU cores
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cores: Option<Vec<ResourcesCPUCore>>,
    ///Current CPU frequency (Mhz)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    ///Minimum CPU frequency (Mhz)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency_minimum: Option<i64>,
    ///Maximum CPU frequency (Mhz)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency_turbo: Option<i64>,
    ///Product name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Socket number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub socket: Option<i64>,
    ///Vendor name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}
///ResourcesCPUThread represents a CPU thread on the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesCPUThread {
    ///Thread ID (used for CPU pinning)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    ///Whether the thread has been isolated (outside of normal scheduling)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub isolated: Option<bool>,
    ///NUMA node the thread is a part of
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numa_node: Option<i64>,
    ///Whether the thread is online (enabled)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,
    ///Thread identifier within the core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread: Option<i64>,
}
///ResourcesGPU represents the GPU resources available on the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesGPU {
    ///List of GPUs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cards: Option<Vec<ResourcesGPUCard>>,
    ///Total number of GPUs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}
///ResourcesGPUCard represents a GPU card on the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesGPUCard {
    ///Kernel driver currently associated with the GPU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    ///Version of the kernel driver
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drm: Option<ResourcesGPUCardDRM>,
    ///Map of available mediated device profiles
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdev: Option<BTreeMap<String, ResourcesGPUCardMdev>>,
    ///NUMA node the GPU is a part of
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numa_node: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nvidia: Option<ResourcesGPUCardNvidia>,
    ///PCI address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pci_address: Option<String>,
    ///Name of the product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    ///PCI ID of the product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sriov: Option<ResourcesGPUCardSRIOV>,
    ///USB address (for USB cards)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usb_address: Option<String>,
    ///Name of the vendor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    ///PCI ID of the vendor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<String>,
}
///ResourcesGPUCardDRM represents the Linux DRM configuration of the GPU
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesGPUCardDRM {
    ///Card device number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_device: Option<String>,
    ///Card device name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_name: Option<String>,
    ///Control device number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_device: Option<String>,
    ///Control device name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_name: Option<String>,
    ///DRM card ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    ///Render device number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub render_device: Option<String>,
    ///Render device name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub render_name: Option<String>,
}
///ResourcesGPUCardMdev represents the mediated devices configuration of the GPU
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesGPUCardMdev {
    ///The mechanism used by this device
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    ///Number of available devices of this profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<i64>,
    ///Profile description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///List of active devices (UUIDs)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<String>>,
    ///Profile name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
///ResourcesGPUCardNvidia represents additional information for NVIDIA GPUs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesGPUCardNvidia {
    ///Architecture (generation)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    ///Brand name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    ///Card device number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_device: Option<String>,
    ///Card device name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_name: Option<String>,
    ///Version of the CUDA API
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cuda_version: Option<String>,
    ///Model name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    ///Version of the NVRM (usually driver version)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nvrm_version: Option<String>,
    ///GPU UUID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
///ResourcesGPUCardSRIOV represents the SRIOV configuration of the GPU
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesGPUCardSRIOV {
    ///Number of VFs currently configured
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_vfs: Option<i64>,
    ///Maximum number of supported VFs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_vfs: Option<i64>,
    ///List of VFs (as additional GPU devices)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vfs: Option<Vec<ResourcesGPUCard>>,
}
///ResourcesMemory represents the memory resources available on the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesMemory {
    ///Size of memory huge pages (bytes)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hugepages_size: Option<i64>,
    ///Total of memory huge pages (bytes)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hugepages_total: Option<i64>,
    ///Used memory huge pages (bytes)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hugepages_used: Option<i64>,
    ///List of NUMA memory nodes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<ResourcesMemoryNode>>,
    ///Total system memory (bytes)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    ///Used system memory (bytes)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
}
///ResourcesMemoryNode represents the node-specific memory resources available on the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesMemoryNode {
    ///Total of memory huge pages (bytes)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hugepages_total: Option<i64>,
    ///Used memory huge pages (bytes)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hugepages_used: Option<i64>,
    ///NUMA node identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numa_node: Option<i64>,
    ///Total system memory (bytes)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    ///Used system memory (bytes)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
}
///ResourcesNetwork represents the network cards available on the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesNetwork {
    ///List of network cards
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cards: Option<Vec<ResourcesNetworkCard>>,
    ///Total number of network cards
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}
///ResourcesNetworkCard represents a network card on the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesNetworkCard {
    ///Kernel driver currently associated with the card
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    ///Version of the kernel driver
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver_version: Option<String>,
    ///Current firmware version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    ///NUMA node the card is a part of
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numa_node: Option<i64>,
    ///PCI address (for PCI cards)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pci_address: Option<String>,
    ///List of ports on the card
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<ResourcesNetworkCardPort>>,
    ///Name of the product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    ///PCI ID of the product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sriov: Option<ResourcesNetworkCardSRIOV>,
    ///USB address (for USB cards)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usb_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vdpa: Option<ResourcesNetworkCardVDPA>,
    ///Name of the vendor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    ///PCI ID of the vendor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<String>,
}
///ResourcesNetworkCardPort represents a network port on the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesNetworkCardPort {
    ///MAC address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    ///Whether auto negotiation is used
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_negotiation: Option<bool>,
    ///Port identifier (interface name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infiniband: Option<ResourcesNetworkCardPortInfiniband>,
    ///Whether a link was detected
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_detected: Option<bool>,
    ///Duplex type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_duplex: Option<String>,
    ///Current speed (Mbit/s)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_speed: Option<i64>,
    ///Port number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    ///Current port type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port_type: Option<String>,
    ///Transport protocol
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    ///List of supported modes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supported_modes: Option<Vec<String>>,
    ///List of supported port types
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supported_ports: Option<Vec<String>>,
    ///Type of transceiver used
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transceiver_type: Option<String>,
}
///ResourcesNetworkCardPortInfiniband represents the Linux Infiniband configuration for the port
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesNetworkCardPortInfiniband {
    ///ISSM device number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issm_device: Option<String>,
    ///ISSM device name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issm_name: Option<String>,
    ///MAD device number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mad_device: Option<String>,
    ///MAD device name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mad_name: Option<String>,
    ///Verb device number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb_device: Option<String>,
    ///Verb device name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb_name: Option<String>,
}
///ResourcesNetworkCardSRIOV represents the SRIOV configuration of the network card
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesNetworkCardSRIOV {
    ///Number of VFs currently configured
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_vfs: Option<i64>,
    ///Maximum number of supported VFs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_vfs: Option<i64>,
    ///List of VFs (as additional Network devices)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vfs: Option<Vec<ResourcesNetworkCard>>,
}
///ResourcesNetworkCardVDPA represents the VDPA configuration of the network card
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesNetworkCardVDPA {
    ///Device identifier of the VDPA device
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    ///Name of the VDPA device
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
///ResourcesPCI represents the PCI devices available on the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesPCI {
    ///List of PCI devices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<ResourcesPCIDevice>>,
    ///Total number of PCI devices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}
///ResourcesPCIDevice represents a PCI device
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesPCIDevice {
    ///Kernel driver currently associated with the GPU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    ///Version of the kernel driver
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver_version: Option<String>,
    ///IOMMU group number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iommu_group: Option<i64>,
    ///NUMA node the card is a part of
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numa_node: Option<i64>,
    ///PCI address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pci_address: Option<String>,
    ///Name of the product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    ///PCI ID of the product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    ///Name of the vendor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    ///PCI ID of the vendor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpd: Option<ResourcesPCIVPD>,
}
///ResourcesPCIVPD represents VPD entries for a device
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesPCIVPD {
    ///Vendor provided key/value pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<BTreeMap<String, String>>,
    ///Hardware provided product name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
}
///ResourcesStorage represents the local storage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesStorage {
    ///List of disks
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<ResourcesStorageDisk>>,
    ///Total number of partitions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}
///ResourcesStorageDisk represents a disk
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesStorageDisk {
    ///Block size
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i64>,
    ///Device number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    ///UUID of the filesystem on the device
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_fs_uuid: Option<String>,
    ///Device by-id identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    ///Device by-path identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_path: Option<String>,
    ///Current firmware version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    ///ID of the disk (device name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Disk model name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    ///Mounted status of the disk
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mounted: Option<bool>,
    ///NUMA node the disk is a part of
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numa_node: Option<i64>,
    ///List of partitions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<ResourcesStorageDiskPartition>>,
    ///PCI address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pci_address: Option<String>,
    ///Whether the disk is read-only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    ///Whether the disk is removable (hot-plug)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removable: Option<bool>,
    ///Rotation speed (RPM)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rpm: Option<i64>,
    ///Serial number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    ///Total size of the disk (bytes)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    ///Storage type
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    ///USB address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usb_address: Option<String>,
    ///Parent device type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_by: Option<String>,
    ///WWN identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wwn: Option<String>,
}
///ResourcesStorageDiskPartition represents a partition on a disk
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesStorageDiskPartition {
    ///Device number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    ///UUID of the filesystem on the device
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_fs_uuid: Option<String>,
    ///ID of the partition (device name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Mounted status of the partition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mounted: Option<bool>,
    ///Partition number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<i64>,
    ///Whether the partition is read-only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    ///Size of the partition (bytes)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}
///ResourcesStoragePool represents the resources available to a given storage pool
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesStoragePool {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inodes: Option<ResourcesStoragePoolInodes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space: Option<ResourcesStoragePoolSpace>,
}
///ResourcesStoragePoolInodes represents the inodes available to a given storage pool
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesStoragePoolInodes {
    ///Total inodes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    ///Used inodes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
}
///ResourcesStoragePoolSpace represents the space available to a given storage pool
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesStoragePoolSpace {
    ///Total disk space (bytes)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    ///Used disk space (bytes)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
}
///ResourcesSystem represents the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesSystem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chassis: Option<ResourcesSystemChassis>,
    ///System family
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware: Option<ResourcesSystemFirmware>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motherboard: Option<ResourcesSystemMotherboard>,
    ///System model
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    ///System serial number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /**System nanufacturer SKU
LENOVO_MT_20HR_BU_Think_FM_ThinkPad X1 Carbon 5th*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    ///System type (unknown, physical, virtual-machine, container, ...)
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    ///System UUID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    ///System vendor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    ///System version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
///ResourcesSystemChassis represents the system chassis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesSystemChassis {
    ///Chassis serial number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    ///Chassis type
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    ///Chassis vendor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    ///Chassis version/revision
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
///ResourcesSystemFirmware represents the system firmware
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesSystemFirmware {
    ///Firmware build date
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    ///Firmware vendor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    ///Firmware version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
///ResourcesSystemMotherboard represents the motherboard
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesSystemMotherboard {
    ///Motherboard model
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    ///Motherboard serial number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    ///Motherboard vendor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    ///Motherboard version/revision
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
///ResourcesUSB represents the USB devices available on the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesUSB {
    ///List of USB devices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<ResourcesUSBDevice>>,
    ///Total number of USB devices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}
///ResourcesUSBDevice represents a USB device
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesUSBDevice {
    ///USB address (bus)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bus_address: Option<i64>,
    ///USB address (device)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_address: Option<i64>,
    ///List of USB interfaces
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<Vec<ResourcesUSBDeviceInterface>>,
    ///Name of the product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    ///USB ID of the product
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    ///USB serial number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    ///Transfer speed (Mbit/s)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    ///Name of the vendor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    ///USB ID of the vendor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<String>,
}
///ResourcesUSBDeviceInterface represents a USB device interface
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesUSBDeviceInterface {
    ///Class of USB interface
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    ///ID of the USB interface class
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class_id: Option<i64>,
    ///Kernel driver currently associated with the device
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    ///Version of the kernel driver
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver_version: Option<String>,
    ///Interface number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    ///Sub class of the interface
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subclass: Option<String>,
    ///ID of the USB interface sub class
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subclass_id: Option<i64>,
}
