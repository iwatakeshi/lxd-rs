//! Example: Using Unix socket to connect to LXD
//!
//! This example demonstrates using the async REST API client
//! to connect to a local LXD server via Unix socket.

use lxd::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== LXD Unix Socket Example ===\n");

    // Create client with Unix socket (default path for snap: /var/snap/lxd/common/lxd/unix.socket)
    let client = Client::new_unix_socket()?;

    // Get server information
    println!("1. Getting server information...");
    match client.get_server().await {
        Ok(server) => {
            #[cfg(feature = "generated")]
            {
                println!(
                    "   ✓ API Version: {}",
                    server.api_version.as_deref().unwrap_or("unknown")
                );
                println!("   ✓ Auth: {}", server.auth.as_deref().unwrap_or("unknown"));
            }
            #[cfg(not(feature = "generated"))]
            {
                println!("   ✓ API Version: {}", server.api_version);
                println!("   ✓ Auth: {}", server.auth);
            }
        }
        Err(e) => {
            eprintln!("   ✗ Error: {}", e);
            eprintln!(
                "\n   Make sure LXD is running and you have permission to access the socket."
            );
            eprintln!("   Try: sudo usermod -aG lxd $USER && newgrp lxd");
            return Ok(());
        }
    }

    // List all instances
    println!("\n2. Listing instances...");
    match client.list_instances_full().await {
        Ok(instances) => {
            println!("   ✓ Found {} instances", instances.len());
            for instance in instances.iter().take(5) {
                #[cfg(feature = "generated")]
                {
                    println!(
                        "     - {} [{}]",
                        instance.name.as_deref().unwrap_or("unknown"),
                        instance.status.as_deref().unwrap_or("unknown")
                    );
                }
                #[cfg(not(feature = "generated"))]
                {
                    println!("     - {} [{}]", instance.name, instance.status);
                }
            }
        }
        Err(e) => eprintln!("   ✗ Error: {}", e),
    }

    // List all images
    println!("\n3. Listing images...");
    match client.list_images_full().await {
        Ok(images) => {
            println!("   ✓ Found {} images", images.len());
            for image in images.iter().take(3) {
                #[cfg(feature = "generated")]
                {
                    if let Some(fp) = &image.fingerprint {
                        println!("     - {}...", &fp[..12.min(fp.len())]);
                    }
                }
                #[cfg(not(feature = "generated"))]
                {
                    println!(
                        "     - {}...",
                        &image.fingerprint[..12.min(image.fingerprint.len())]
                    );
                }
            }
        }
        Err(e) => eprintln!("   ✗ Error: {}", e),
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
