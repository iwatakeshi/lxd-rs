//! # LXD Client
//!
//! Async REST API client for LXD with support for Unix socket and HTTPS transports.
//!
//! ## Example
//!
//! ```rust,no_run
//! use lxd_client::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new_unix_socket()?;
//!     let instances = client.list_instances().await?;
//!     println!("Found {} instances", instances.len());
//!     Ok(())
//! }
//! ```

mod config;
mod error;
mod transport;
mod endpoints;

pub use config::{ClientConfig, ClientBuilder};
pub use error::{Error, Result};
pub use transport::{Transport, TransportKind, UnixSocketTransport, HttpsTransport};

use lxd_types::{
    Instance, InstancesPost, InstancePut, InstanceState, InstanceStatePut,
    Image, ImagesPost, ImagePut,
    Operation, Response, Server,
    Profile, ProfilesPost, ProfilePut,
    Project, ProjectsPost, ProjectPut,
    Network, NetworksPost, NetworkPut,
    StoragePool, StoragePoolsPost, StoragePoolPut,
    StorageVolume, StorageVolumesPost, StorageVolumePut,
    InstanceSnapshot, InstanceSnapshotsPost,
};
use std::path::Path;
use std::time::Duration;

/// LXD API client
pub struct Client {
    transport: TransportKind,
    project: Option<String>,
    config: ClientConfig,
}

impl Client {
    /// Create a new client using Unix socket transport
    pub fn new_unix_socket() -> Result<Self> {
        Self::new_unix_socket_path("/var/snap/lxd/common/lxd/unix.socket")
    }
    
    /// Create a new client using Unix socket at a specific path
    pub fn new_unix_socket_path(path: impl AsRef<Path>) -> Result<Self> {
        let transport = UnixSocketTransport::new(path)?;
        Ok(Self {
            transport: TransportKind::UnixSocket(transport),
            project: None,
            config: ClientConfig::default(),
        })
    }
    
    /// Create a new client using HTTPS transport
    pub fn new_https(
        url: impl Into<String>,
        cert_path: impl AsRef<Path>,
        key_path: impl AsRef<Path>,
    ) -> Result<Self> {
        let transport = HttpsTransport::new(url, cert_path, key_path)?;
        Ok(Self {
            transport: TransportKind::Https(transport),
            project: None,
            config: ClientConfig::default(),
        })
    }
    
    /// Create a new client from a builder
    pub fn from_builder(builder: ClientBuilder, transport: TransportKind) -> Self {
        Self {
            transport,
            project: builder.get_project().map(|s| s.to_string()),
            config: builder.get_config().clone(),
        }
    }
    
    /// Set the project for all requests
    pub fn with_project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(project.into());
        self
    }
    
    /// Set the client configuration
    pub fn with_config(mut self, config: ClientConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Set the request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }
    
    /// Set the number of retries
    pub fn with_retries(mut self, retries: u32) -> Self {
        self.config.retries = retries;
        self
    }
    
    /// Get the current configuration
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }
    
    /// Build a path with optional project query parameter
    fn path(&self, base: &str) -> String {
        match &self.project {
            Some(project) => {
                if base.contains('?') {
                    format!("{}&project={}", base, project)
                } else {
                    format!("{}?project={}", base, project)
                }
            }
            None => base.to_string(),
        }
    }
    
    // Server endpoints
    
    /// Get server information
    pub async fn get_server(&self) -> Result<Server> {
        let response: Response<Server> = self.transport.get("/1.0").await?;
        Ok(response.metadata)
    }
    
    /// Get API version
    pub async fn get_api_version(&self) -> Result<String> {
        let server = self.get_server().await?;
        // Handle both generated (Option<String>) and manual (String) types
        #[cfg(feature = "generated")]
        {
            Ok(server.api_version.unwrap_or_default())
        }
        #[cfg(not(feature = "generated"))]
        {
            Ok(server.api_version)
        }
    }
    
    // Instance endpoints
    
    /// List all instances (URLs)
    pub async fn list_instances(&self) -> Result<Vec<String>> {
        let path = self.path("/1.0/instances");
        let response: Response<Vec<String>> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// List all instances with full details
    pub async fn list_instances_full(&self) -> Result<Vec<Instance>> {
        let path = self.path("/1.0/instances?recursion=1");
        let response: Response<Vec<Instance>> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Get a specific instance
    pub async fn get_instance(&self, name: &str) -> Result<Instance> {
        let path = self.path(&format!("/1.0/instances/{}", name));
        let response: Response<Instance> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Create a new instance
    pub async fn create_instance(&self, request: &InstancesPost) -> Result<Operation> {
        let path = self.path("/1.0/instances");
        let response: Response<Operation> = self.transport.post(&path, request).await?;
        Ok(response.metadata)
    }
    
    /// Update an instance
    pub async fn update_instance(&self, name: &str, request: &InstancePut) -> Result<()> {
        let path = self.path(&format!("/1.0/instances/{}", name));
        let _response: Response<()> = self.transport.put(&path, request).await?;
        Ok(())
    }
    
    /// Delete an instance
    pub async fn delete_instance(&self, name: &str) -> Result<Operation> {
        let path = self.path(&format!("/1.0/instances/{}", name));
        let response: Response<Operation> = self.transport.delete(&path).await?;
        Ok(response.metadata)
    }
    
    /// Get instance state
    pub async fn get_instance_state(&self, name: &str) -> Result<InstanceState> {
        let path = self.path(&format!("/1.0/instances/{}/state", name));
        let response: Response<InstanceState> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Change instance state (start, stop, restart, etc.)
    pub async fn change_instance_state(&self, name: &str, request: &InstanceStatePut) -> Result<Operation> {
        let path = self.path(&format!("/1.0/instances/{}/state", name));
        let response: Response<Operation> = self.transport.put(&path, request).await?;
        Ok(response.metadata)
    }
    
    // Convenience methods for instance state
    // These are implemented differently based on whether generated types are used

    /// Start an instance
    pub async fn start_instance(&self, name: &str) -> Result<Operation> {
        #[cfg(feature = "generated")]
        let request = InstanceStatePut {
            action: Some("start".to_string()),
            timeout: None,
            force: None,
            stateful: None,
        };
        #[cfg(not(feature = "generated"))]
        let request = InstanceStatePut {
            action: lxd_types::InstanceAction::Start,
            timeout: None,
            force: None,
            stateful: None,
        };
        self.change_instance_state(name, &request).await
    }
    
    /// Stop an instance
    pub async fn stop_instance(&self, name: &str, force: bool) -> Result<Operation> {
        #[cfg(feature = "generated")]
        let request = InstanceStatePut {
            action: Some("stop".to_string()),
            timeout: Some(30),
            force: Some(force),
            stateful: None,
        };
        #[cfg(not(feature = "generated"))]
        let request = InstanceStatePut {
            action: lxd_types::InstanceAction::Stop,
            timeout: Some(30),
            force: Some(force),
            stateful: None,
        };
        self.change_instance_state(name, &request).await
    }
    
    /// Restart an instance
    pub async fn restart_instance(&self, name: &str) -> Result<Operation> {
        #[cfg(feature = "generated")]
        let request = InstanceStatePut {
            action: Some("restart".to_string()),
            timeout: Some(30),
            force: None,
            stateful: None,
        };
        #[cfg(not(feature = "generated"))]
        let request = InstanceStatePut {
            action: lxd_types::InstanceAction::Restart,
            timeout: Some(30),
            force: None,
            stateful: None,
        };
        self.change_instance_state(name, &request).await
    }
    
    /// Freeze an instance
    pub async fn freeze_instance(&self, name: &str) -> Result<Operation> {
        #[cfg(feature = "generated")]
        let request = InstanceStatePut {
            action: Some("freeze".to_string()),
            timeout: None,
            force: None,
            stateful: None,
        };
        #[cfg(not(feature = "generated"))]
        let request = InstanceStatePut {
            action: lxd_types::InstanceAction::Freeze,
            timeout: None,
            force: None,
            stateful: None,
        };
        self.change_instance_state(name, &request).await
    }
    
    /// Unfreeze an instance
    pub async fn unfreeze_instance(&self, name: &str) -> Result<Operation> {
        #[cfg(feature = "generated")]
        let request = InstanceStatePut {
            action: Some("unfreeze".to_string()),
            timeout: None,
            force: None,
            stateful: None,
        };
        #[cfg(not(feature = "generated"))]
        let request = InstanceStatePut {
            action: lxd_types::InstanceAction::Unfreeze,
            timeout: None,
            force: None,
            stateful: None,
        };
        self.change_instance_state(name, &request).await
    }
    
    // Image endpoints
    
    /// List all images (URLs)
    pub async fn list_images(&self) -> Result<Vec<String>> {
        let path = self.path("/1.0/images");
        let response: Response<Vec<String>> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// List all images with full details
    pub async fn list_images_full(&self) -> Result<Vec<Image>> {
        let path = self.path("/1.0/images?recursion=1");
        let response: Response<Vec<Image>> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Get a specific image
    pub async fn get_image(&self, fingerprint: &str) -> Result<Image> {
        let path = self.path(&format!("/1.0/images/{}", fingerprint));
        let response: Response<Image> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Create/import an image
    pub async fn create_image(&self, request: &ImagesPost) -> Result<Operation> {
        let path = self.path("/1.0/images");
        let response: Response<Operation> = self.transport.post(&path, request).await?;
        Ok(response.metadata)
    }
    
    /// Update an image
    pub async fn update_image(&self, fingerprint: &str, request: &ImagePut) -> Result<()> {
        let path = self.path(&format!("/1.0/images/{}", fingerprint));
        let _response: Response<()> = self.transport.put(&path, request).await?;
        Ok(())
    }
    
    /// Delete an image
    pub async fn delete_image(&self, fingerprint: &str) -> Result<Operation> {
        let path = self.path(&format!("/1.0/images/{}", fingerprint));
        let response: Response<Operation> = self.transport.delete(&path).await?;
        Ok(response.metadata)
    }
    
    // Operation endpoints
    
    /// List all operations (URLs)
    pub async fn list_operations(&self) -> Result<Vec<String>> {
        let response: Response<Vec<String>> = self.transport.get("/1.0/operations").await?;
        Ok(response.metadata)
    }
    
    /// Get a specific operation
    pub async fn get_operation(&self, id: &str) -> Result<Operation> {
        let response: Response<Operation> = self.transport.get(&format!("/1.0/operations/{}", id)).await?;
        Ok(response.metadata)
    }
    
    /// Wait for an operation to complete
    pub async fn wait_operation(&self, id: &str, timeout: Option<i64>) -> Result<Operation> {
        let path = match timeout {
            Some(t) => format!("/1.0/operations/{}/wait?timeout={}", id, t),
            None => format!("/1.0/operations/{}/wait", id),
        };
        let response: Response<Operation> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Cancel an operation
    pub async fn cancel_operation(&self, id: &str) -> Result<()> {
        let _response: Response<()> = self.transport.delete(&format!("/1.0/operations/{}", id)).await?;
        Ok(())
    }
    
    // Profile endpoints
    
    /// List all profiles (URLs)
    pub async fn list_profiles(&self) -> Result<Vec<String>> {
        let path = self.path("/1.0/profiles");
        let response: Response<Vec<String>> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// List all profiles with full details
    pub async fn list_profiles_full(&self) -> Result<Vec<Profile>> {
        let path = self.path("/1.0/profiles?recursion=1");
        let response: Response<Vec<Profile>> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Get a specific profile
    pub async fn get_profile(&self, name: &str) -> Result<Profile> {
        let path = self.path(&format!("/1.0/profiles/{}", name));
        let response: Response<Profile> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Create a new profile
    pub async fn create_profile(&self, request: &ProfilesPost) -> Result<()> {
        let path = self.path("/1.0/profiles");
        let _response: Response<()> = self.transport.post(&path, request).await?;
        Ok(())
    }
    
    /// Update a profile
    pub async fn update_profile(&self, name: &str, request: &ProfilePut) -> Result<()> {
        let path = self.path(&format!("/1.0/profiles/{}", name));
        let _response: Response<()> = self.transport.put(&path, request).await?;
        Ok(())
    }
    
    /// Delete a profile
    pub async fn delete_profile(&self, name: &str) -> Result<()> {
        let path = self.path(&format!("/1.0/profiles/{}", name));
        let _response: Response<()> = self.transport.delete(&path).await?;
        Ok(())
    }
    
    // Project endpoints
    
    /// List all projects (URLs)
    pub async fn list_projects(&self) -> Result<Vec<String>> {
        let response: Response<Vec<String>> = self.transport.get("/1.0/projects").await?;
        Ok(response.metadata)
    }
    
    /// List all projects with full details
    pub async fn list_projects_full(&self) -> Result<Vec<Project>> {
        let response: Response<Vec<Project>> = self.transport.get("/1.0/projects?recursion=1").await?;
        Ok(response.metadata)
    }
    
    /// Get a specific project
    pub async fn get_project(&self, name: &str) -> Result<Project> {
        let response: Response<Project> = self.transport.get(&format!("/1.0/projects/{}", name)).await?;
        Ok(response.metadata)
    }
    
    /// Create a new project
    pub async fn create_project(&self, request: &ProjectsPost) -> Result<()> {
        let _response: Response<()> = self.transport.post("/1.0/projects", request).await?;
        Ok(())
    }
    
    /// Update a project
    pub async fn update_project(&self, name: &str, request: &ProjectPut) -> Result<()> {
        let _response: Response<()> = self.transport.put(&format!("/1.0/projects/{}", name), request).await?;
        Ok(())
    }
    
    /// Delete a project
    pub async fn delete_project(&self, name: &str) -> Result<()> {
        let _response: Response<()> = self.transport.delete(&format!("/1.0/projects/{}", name)).await?;
        Ok(())
    }
    
    // Network endpoints
    
    /// List all networks (URLs)
    pub async fn list_networks(&self) -> Result<Vec<String>> {
        let path = self.path("/1.0/networks");
        let response: Response<Vec<String>> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// List all networks with full details
    pub async fn list_networks_full(&self) -> Result<Vec<Network>> {
        let path = self.path("/1.0/networks?recursion=1");
        let response: Response<Vec<Network>> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Get a specific network
    pub async fn get_network(&self, name: &str) -> Result<Network> {
        let path = self.path(&format!("/1.0/networks/{}", name));
        let response: Response<Network> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Create a new network
    pub async fn create_network(&self, request: &NetworksPost) -> Result<()> {
        let path = self.path("/1.0/networks");
        let _response: Response<()> = self.transport.post(&path, request).await?;
        Ok(())
    }
    
    /// Update a network
    pub async fn update_network(&self, name: &str, request: &NetworkPut) -> Result<()> {
        let path = self.path(&format!("/1.0/networks/{}", name));
        let _response: Response<()> = self.transport.put(&path, request).await?;
        Ok(())
    }
    
    /// Delete a network
    pub async fn delete_network(&self, name: &str) -> Result<()> {
        let path = self.path(&format!("/1.0/networks/{}", name));
        let _response: Response<()> = self.transport.delete(&path).await?;
        Ok(())
    }
    
    // Storage pool endpoints
    
    /// List all storage pools (URLs)
    pub async fn list_storage_pools(&self) -> Result<Vec<String>> {
        let path = self.path("/1.0/storage-pools");
        let response: Response<Vec<String>> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// List all storage pools with full details
    pub async fn list_storage_pools_full(&self) -> Result<Vec<StoragePool>> {
        let path = self.path("/1.0/storage-pools?recursion=1");
        let response: Response<Vec<StoragePool>> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Get a specific storage pool
    pub async fn get_storage_pool(&self, name: &str) -> Result<StoragePool> {
        let path = self.path(&format!("/1.0/storage-pools/{}", name));
        let response: Response<StoragePool> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Create a new storage pool
    pub async fn create_storage_pool(&self, request: &StoragePoolsPost) -> Result<()> {
        let path = self.path("/1.0/storage-pools");
        let _response: Response<()> = self.transport.post(&path, request).await?;
        Ok(())
    }
    
    /// Update a storage pool
    pub async fn update_storage_pool(&self, name: &str, request: &StoragePoolPut) -> Result<()> {
        let path = self.path(&format!("/1.0/storage-pools/{}", name));
        let _response: Response<()> = self.transport.put(&path, request).await?;
        Ok(())
    }
    
    /// Delete a storage pool
    pub async fn delete_storage_pool(&self, name: &str) -> Result<()> {
        let path = self.path(&format!("/1.0/storage-pools/{}", name));
        let _response: Response<()> = self.transport.delete(&path).await?;
        Ok(())
    }
    
    // Storage volume endpoints
    
    /// List all volumes in a storage pool (URLs)
    pub async fn list_storage_volumes(&self, pool: &str) -> Result<Vec<String>> {
        let path = self.path(&format!("/1.0/storage-pools/{}/volumes", pool));
        let response: Response<Vec<String>> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// List all volumes in a storage pool with full details
    pub async fn list_storage_volumes_full(&self, pool: &str) -> Result<Vec<StorageVolume>> {
        let path = self.path(&format!("/1.0/storage-pools/{}/volumes?recursion=1", pool));
        let response: Response<Vec<StorageVolume>> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Get a specific storage volume
    pub async fn get_storage_volume(&self, pool: &str, volume_type: &str, name: &str) -> Result<StorageVolume> {
        let path = self.path(&format!("/1.0/storage-pools/{}/volumes/{}/{}", pool, volume_type, name));
        let response: Response<StorageVolume> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Create a new storage volume
    pub async fn create_storage_volume(&self, pool: &str, request: &StorageVolumesPost) -> Result<()> {
        let path = self.path(&format!("/1.0/storage-pools/{}/volumes", pool));
        let _response: Response<()> = self.transport.post(&path, request).await?;
        Ok(())
    }
    
    /// Update a storage volume
    pub async fn update_storage_volume(&self, pool: &str, volume_type: &str, name: &str, request: &StorageVolumePut) -> Result<()> {
        let path = self.path(&format!("/1.0/storage-pools/{}/volumes/{}/{}", pool, volume_type, name));
        let _response: Response<()> = self.transport.put(&path, request).await?;
        Ok(())
    }
    
    /// Delete a storage volume
    pub async fn delete_storage_volume(&self, pool: &str, volume_type: &str, name: &str) -> Result<()> {
        let path = self.path(&format!("/1.0/storage-pools/{}/volumes/{}/{}", pool, volume_type, name));
        let _response: Response<()> = self.transport.delete(&path).await?;
        Ok(())
    }
    
    // Instance snapshot endpoints
    
    /// List all snapshots for an instance (URLs)
    pub async fn list_instance_snapshots(&self, instance: &str) -> Result<Vec<String>> {
        let path = self.path(&format!("/1.0/instances/{}/snapshots", instance));
        let response: Response<Vec<String>> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// List all snapshots for an instance with full details
    pub async fn list_instance_snapshots_full(&self, instance: &str) -> Result<Vec<InstanceSnapshot>> {
        let path = self.path(&format!("/1.0/instances/{}/snapshots?recursion=1", instance));
        let response: Response<Vec<InstanceSnapshot>> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Get a specific instance snapshot
    pub async fn get_instance_snapshot(&self, instance: &str, snapshot: &str) -> Result<InstanceSnapshot> {
        let path = self.path(&format!("/1.0/instances/{}/snapshots/{}", instance, snapshot));
        let response: Response<InstanceSnapshot> = self.transport.get(&path).await?;
        Ok(response.metadata)
    }
    
    /// Create a new instance snapshot
    pub async fn create_instance_snapshot(&self, instance: &str, request: &InstanceSnapshotsPost) -> Result<Operation> {
        let path = self.path(&format!("/1.0/instances/{}/snapshots", instance));
        let response: Response<Operation> = self.transport.post(&path, request).await?;
        Ok(response.metadata)
    }
    
    /// Delete an instance snapshot
    pub async fn delete_instance_snapshot(&self, instance: &str, snapshot: &str) -> Result<Operation> {
        let path = self.path(&format!("/1.0/instances/{}/snapshots/{}", instance, snapshot));
        let response: Response<Operation> = self.transport.delete(&path).await?;
        Ok(response.metadata)
    }
    
    /// Restore an instance from a snapshot
    pub async fn restore_instance_snapshot(&self, instance: &str, snapshot: &str) -> Result<Operation> {
        let path = self.path(&format!("/1.0/instances/{}", instance));
        let request = serde_json::json!({
            "restore": snapshot
        });
        let response: Response<Operation> = self.transport.put(&path, &request).await?;
        Ok(response.metadata)
    }
}
