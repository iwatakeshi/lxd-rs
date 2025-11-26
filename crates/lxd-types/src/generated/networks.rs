//!//! Networks types for LXD API
#![allow(clippy::derive_partial_eq_without_eq)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[allow(unused_imports)]
use super::*;
///Network represents a LXD network
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Network {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///Network configuration map (refer to doc/networks.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Cluster members on which the network has been defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<String>>,
    ///Whether this is a LXD managed network
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    ///The network name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Project name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///The state of the network (for managed network in clusters)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///The network type
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    ///List of URLs of objects using this profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_by: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkACL {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///ACL configuration map (refer to doc/network-acls.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the ACL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///List of egress rules (order independent)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egress: Option<Vec<NetworkACLRule>>,
    ///List of ingress rules (order independent)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<NetworkACLRule>>,
    ///The new name for the ACL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Project name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///List of URLs of objects using this profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_by: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkACLPost {
    ///The new name for the ACL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkACLPut {
    ///ACL configuration map (refer to doc/network-acls.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the ACL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///List of egress rules (order independent)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egress: Option<Vec<NetworkACLRule>>,
    ///List of ingress rules (order independent)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<NetworkACLRule>>,
}
///Refer to doc/network-acls.md for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkACLRule {
    ///Action to perform on rule match
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    ///Description of the rule
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Destination address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    ///Destination port
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<String>,
    ///ICMP message code (for ICMP protocol)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icmp_code: Option<String>,
    ///Type of ICMP message (for ICMP protocol)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icmp_type: Option<String>,
    ///Protocol
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    ///Source address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    ///Source port
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_port: Option<String>,
    ///State of the rule
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkACLsPost {
    ///ACL configuration map (refer to doc/network-acls.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the ACL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///List of egress rules (order independent)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egress: Option<Vec<NetworkACLRule>>,
    ///List of ingress rules (order independent)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<NetworkACLRule>>,
    ///The new name for the ACL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/**NetworkAllocations used for displaying network addresses used by a consuming entity
e.g, instance, network forward, load-balancer, network...*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkAllocations {
    ///The network address of the allocation (in CIDR format)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<String>,
    ///Hwaddr is the MAC address of the entity consuming the network address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hwaddr: Option<String>,
    ///Whether the entity comes from a network that LXD performs egress source NAT on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nat: Option<bool>,
    ///Network is the name of the network the allocated address belongs to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    ///Type of the entity consuming the network address
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    ///Name of the entity consuming the network address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_by: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkForward {
    ///Forward configuration map (refer to doc/network-forwards.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the forward listen IP
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The listen address of the forward
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen_address: Option<String>,
    ///What cluster member this record was found on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    ///Port forwards (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<NetworkForwardPort>>,
}
///NetworkForwardPort represents a port specification in a network address forward
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkForwardPort {
    ///Description of the forward port
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///ListenPort(s) to forward (comma delimited ranges)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen_port: Option<String>,
    ///Protocol for port forward (either tcp or udp)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    ///TargetAddress to forward ListenPorts to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_address: Option<String>,
    ///TargetPort(s) to forward ListenPorts to (allows for many-to-one)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_port: Option<String>,
}
///NetworkForwardPut represents the modifiable fields of a LXD network address forward
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkForwardPut {
    ///Forward configuration map (refer to doc/network-forwards.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the forward listen IP
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Port forwards (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<NetworkForwardPort>>,
}
///NetworkForwardsPost represents the fields of a new LXD network address forward
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkForwardsPost {
    ///Forward configuration map (refer to doc/network-forwards.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the forward listen IP
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /**The listen address of the forward
For OVN networks only, you can dynamically allocate the listen address from a pre-defined range.
To do so for an IPv4 address, provide a listen_address of `0.0.0.0`.
For an IPv6 address, provide a listen_address of `::`.
These are equivalent to the `allocate=ipv{4|6}` flag used to create a network forward via the CLI.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen_address: Option<String>,
    ///Port forwards (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<NetworkForwardPort>>,
}
///NetworkLease represents a DHCP lease
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkLease {
    ///The IP address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    ///The hostname associated with the record
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    ///The MAC address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hwaddr: Option<String>,
    ///What cluster member this record was found on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    ///Name of the project of the entity related to the hostname
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///The type of record (static or dynamic)
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
///NetworkLoadBalancer used for displaying a network load balancer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkLoadBalancer {
    ///Backends (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<NetworkLoadBalancerBackend>>,
    ///Load balancer configuration map (refer to doc/network-load-balancers.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the load balancer listen IP
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The listen address of the load balancer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen_address: Option<String>,
    ///What cluster member this record was found on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    ///Port forwards (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<NetworkLoadBalancerPort>>,
}
///NetworkLoadBalancerBackend represents a target backend specification in a network load balancer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkLoadBalancerBackend {
    ///Description of the load balancer backend
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name of the load balancer backend
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///TargetAddress to forward ListenPorts to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_address: Option<String>,
    ///TargetPort(s) to forward ListenPorts to (allows for many-to-one)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_port: Option<String>,
}
///NetworkLoadBalancerPort represents a port specification in a network load balancer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkLoadBalancerPort {
    ///Description of the load balancer port
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///ListenPort(s) of load balancer (comma delimited ranges)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen_port: Option<String>,
    ///Protocol for load balancer port (either tcp or udp)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    ///TargetBackend backend names to load balance ListenPorts to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_backend: Option<Vec<String>>,
}
///NetworkLoadBalancerPut represents the modifiable fields of a LXD network load balancer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkLoadBalancerPut {
    ///Backends (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<NetworkLoadBalancerBackend>>,
    ///Load balancer configuration map (refer to doc/network-load-balancers.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the load balancer listen IP
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Port forwards (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<NetworkLoadBalancerPort>>,
}
///NetworkLoadBalancersPost represents the fields of a new LXD network load balancer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkLoadBalancersPost {
    ///Backends (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backends: Option<Vec<NetworkLoadBalancerBackend>>,
    ///Load balancer configuration map (refer to doc/network-load-balancers.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the load balancer listen IP
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The listen address of the load balancer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen_address: Option<String>,
    ///Port forwards (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<NetworkLoadBalancerPort>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkPeer {
    ///Peer configuration map (refer to doc/network-peers.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the peer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name of the peer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The state of the peering
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///Name of the target network
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_network: Option<String>,
    ///Name of the target project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_project: Option<String>,
    ///List of URLs of objects using this network peering
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_by: Option<Vec<String>>,
}
///NetworkPeerPut represents the modifiable fields of a LXD network peering
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkPeerPut {
    ///Peer configuration map (refer to doc/network-peers.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the peer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
///NetworkPeersPost represents the fields of a new LXD network peering
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkPeersPost {
    ///Peer configuration map (refer to doc/network-peers.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the peer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name of the peer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Name of the target network
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_network: Option<String>,
    ///Name of the target project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_project: Option<String>,
}
///NetworkPost represents the fields required to rename a LXD network
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkPost {
    ///The new name for the network
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
///NetworkPut represents the modifiable fields of a LXD network
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkPut {
    ///Network configuration map (refer to doc/networks.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
///NetworkState represents the network state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkState {
    ///List of addresses
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<NetworkStateAddress>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bond: Option<NetworkStateBond>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bridge: Option<NetworkStateBridge>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counters: Option<NetworkStateCounters>,
    ///MAC address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hwaddr: Option<String>,
    ///MTU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ovn: Option<NetworkStateOVN>,
    ///Link state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    ///Interface type
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vlan: Option<NetworkStateVLAN>,
}
///NetworkStateAddress represents a network address
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkStateAddress {
    ///IP address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    ///Address family
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    ///IP netmask (CIDR)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,
    ///Address scope
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
///NetworkStateBond represents bond specific state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkStateBond {
    ///Delay on link down (ms)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub down_delay: Option<i64>,
    ///List of devices that are part of the bond
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower_devices: Option<Vec<String>>,
    ///How often to check for link state (ms)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mii_frequency: Option<i64>,
    ///Bond link state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mii_state: Option<String>,
    ///Bonding mode
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    ///Transmit balancing policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmit_policy: Option<String>,
    ///Delay on link up (ms)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub up_delay: Option<i64>,
}
///NetworkStateBridge represents bridge specific state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkStateBridge {
    ///Delay on port join (ms)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forward_delay: Option<i64>,
    ///Bridge ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Whether STP is enabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stp: Option<bool>,
    ///List of devices that are in the bridge
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper_devices: Option<Vec<String>>,
    ///Default VLAN ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vlan_default: Option<i64>,
    ///Whether VLAN filtering is enabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vlan_filtering: Option<bool>,
}
///NetworkStateCounters represents packet counters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkStateCounters {
    ///Number of bytes received
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes_received: Option<i64>,
    ///Number of bytes sent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes_sent: Option<i64>,
    ///Number of packets received
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packets_received: Option<i64>,
    ///Number of packets sent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packets_sent: Option<i64>,
}
///NetworkStateOVN represents OVN specific state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkStateOVN {
    ///OVN network chassis name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chassis: Option<String>,
}
///NetworkStateVLAN represents VLAN specific state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkStateVLAN {
    ///Parent device
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower_device: Option<String>,
    ///VLAN ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vid: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkZone {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///Zone configuration map (refer to doc/network-zones.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the network zone
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The name of the zone (DNS domain name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Project name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///List of URLs of objects using this network zone
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_by: Option<Vec<String>>,
}
///NetworkZonePut represents the modifiable fields of a LXD network zone
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkZonePut {
    ///Zone configuration map (refer to doc/network-zones.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the network zone
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkZoneRecord {
    ///Advanced configuration for the record
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the record
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Entries in the record
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<NetworkZoneRecordEntry>>,
    ///The name of the record
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
///NetworkZoneRecordEntry represents the fields in a record entry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkZoneRecordEntry {
    ///TTL for the entry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    ///Type of DNS entry
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    ///Value for the record
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
///NetworkZoneRecordPut represents the modifiable fields of a LXD network zone record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkZoneRecordPut {
    ///Advanced configuration for the record
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the record
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Entries in the record
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<NetworkZoneRecordEntry>>,
}
///NetworkZoneRecordsPost represents the fields of a new LXD network zone record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkZoneRecordsPost {
    ///Advanced configuration for the record
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the record
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Entries in the record
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<NetworkZoneRecordEntry>>,
    ///The record name in the zone
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
///NetworkZonesPost represents the fields of a new LXD network zone
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkZonesPost {
    ///Zone configuration map (refer to doc/network-zones.md)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    ///Description of the network zone
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The name of the zone (DNS domain name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
///NetworksPost represents the fields of a new LXD network
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworksPost {
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
