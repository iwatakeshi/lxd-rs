//! Integration tests for LXD client
//!
//! These tests require a running LXD daemon and appropriate permissions.
//! They are designed to gracefully skip if LXD is not available.

use lxd_client::Client;

#[tokio::test]
async fn test_unix_socket_connection() {
    let client = match Client::new_unix_socket() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Skipping test - Cannot create client: {}", e);
            return;
        }
    };

    match client.get_server().await {
        Ok(server) => {
            // With generated types, fields are Option<T>
            #[cfg(feature = "generated")]
            {
                assert!(server.api_version.is_some());
                println!("Connected to LXD API version: {:?}", server.api_version);
            }
            #[cfg(not(feature = "generated"))]
            {
                assert!(!server.api_version.is_empty());
                println!("Connected to LXD API version: {}", server.api_version);
            }
        }
        Err(e) => {
            eprintln!("Skipping test - LXD not available: {}", e);
        }
    }
}

#[tokio::test]
async fn test_get_api_version() {
    let client = match Client::new_unix_socket() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Skipping test - Cannot create client: {}", e);
            return;
        }
    };

    match client.get_api_version().await {
        Ok(version) => {
            println!("LXD API version: {}", version);
            assert!(!version.is_empty());
        }
        Err(e) => {
            eprintln!("Skipping test - LXD not available: {}", e);
        }
    }
}

#[tokio::test]
async fn test_list_instances() {
    let client = match Client::new_unix_socket() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Skipping test - Cannot create client: {}", e);
            return;
        }
    };

    match client.list_instances().await {
        Ok(instances) => {
            println!("Found {} instance URLs", instances.len());
            for url in &instances {
                println!("  - {}", url);
            }
        }
        Err(e) => {
            eprintln!("Skipping test - LXD not available: {}", e);
        }
    }
}

#[tokio::test]
async fn test_list_instances_full() {
    let client = match Client::new_unix_socket() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Skipping test - Cannot create client: {}", e);
            return;
        }
    };

    match client.list_instances_full().await {
        Ok(instances) => {
            println!("Found {} instances", instances.len());
            for instance in &instances {
                #[cfg(feature = "generated")]
                {
                    println!(
                        "  - {} ({})",
                        instance.name.as_deref().unwrap_or("unknown"),
                        instance.status.as_deref().unwrap_or("unknown")
                    );
                }
                #[cfg(not(feature = "generated"))]
                {
                    println!("  - {} ({})", instance.name, instance.status);
                }
            }
        }
        Err(e) => {
            eprintln!("Skipping test - LXD not available: {}", e);
        }
    }
}

#[tokio::test]
async fn test_list_images() {
    let client = match Client::new_unix_socket() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Skipping test - Cannot create client: {}", e);
            return;
        }
    };

    match client.list_images().await {
        Ok(images) => {
            println!("Found {} image URLs", images.len());
        }
        Err(e) => {
            eprintln!("Skipping test - LXD not available: {}", e);
        }
    }
}

#[tokio::test]
async fn test_get_nonexistent_instance() {
    let client = match Client::new_unix_socket() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Skipping test - Cannot create client: {}", e);
            return;
        }
    };

    // This should return an error for a nonexistent instance
    match client.get_instance("nonexistent-instance-xyz-12345").await {
        Ok(_) => {
            panic!("Expected error for nonexistent instance");
        }
        Err(e) => {
            println!("Got expected error: {}", e);
        }
    }
}

#[tokio::test]
async fn test_project_support() {
    let client = match Client::new_unix_socket() {
        Ok(c) => c.with_project("default"),
        Err(e) => {
            eprintln!("Skipping test - Cannot create client: {}", e);
            return;
        }
    };

    match client.list_instances().await {
        Ok(instances) => {
            println!("Found {} instances in 'default' project", instances.len());
        }
        Err(e) => {
            eprintln!("Skipping test - LXD not available: {}", e);
        }
    }
}
