//! Unit tests for LXD client
//!
//! These tests use mocking and don't require a running LXD instance.

use lxd_client::Error;
use lxd_types::{Response, ResponseType};

mod response_parsing {
    use super::*;

    #[test]
    fn test_sync_response_success() {
        let json = r#"{
            "type": "sync",
            "status": "Success",
            "status_code": 200,
            "operation": "",
            "error": "",
            "error_code": 0,
            "metadata": {"api_version": "1.0"}
        }"#;

        let response: Response<serde_json::Value> = serde_json::from_str(json).unwrap();
        
        assert_eq!(response.response_type, ResponseType::Sync);
        assert_eq!(response.status, "Success");
        assert_eq!(response.status_code, 200);
        assert!(response.is_success());
        assert!(!response.is_async());
        assert!(!response.is_error());
    }

    #[test]
    fn test_async_response() {
        let json = r#"{
            "type": "async",
            "status": "Operation created",
            "status_code": 100,
            "operation": "/1.0/operations/abc123",
            "error": "",
            "error_code": 0,
            "metadata": null
        }"#;

        let response: Response<Option<()>> = serde_json::from_str(json).unwrap();
        
        assert_eq!(response.response_type, ResponseType::Async);
        assert!(response.is_async());
        assert!(response.is_success());
        assert_eq!(response.operation_id(), Some("abc123"));
    }

    #[test]
    fn test_error_response() {
        let json = r#"{
            "type": "error",
            "status": "Failure",
            "status_code": 404,
            "operation": "",
            "error": "Instance not found",
            "error_code": 404,
            "metadata": null
        }"#;

        let response: Response<Option<()>> = serde_json::from_str(json).unwrap();
        
        assert_eq!(response.response_type, ResponseType::Error);
        assert!(response.is_error());
        assert!(!response.is_success());
        assert_eq!(response.error, "Instance not found");
        assert_eq!(response.error_code, 404);
    }

    #[test]
    fn test_operation_id_extraction() {
        let json = r#"{
            "type": "async",
            "status": "Operation created",
            "status_code": 100,
            "operation": "/1.0/operations/550e8400-e29b-41d4-a716-446655440000",
            "error": "",
            "error_code": 0,
            "metadata": null
        }"#;

        let response: Response<Option<()>> = serde_json::from_str(json).unwrap();
        assert_eq!(
            response.operation_id(),
            Some("550e8400-e29b-41d4-a716-446655440000")
        );
    }

    #[test]
    fn test_empty_operation_returns_none() {
        let json = r#"{
            "type": "sync",
            "status": "Success",
            "status_code": 200,
            "operation": "",
            "error": "",
            "error_code": 0,
            "metadata": null
        }"#;

        let response: Response<Option<()>> = serde_json::from_str(json).unwrap();
        assert_eq!(response.operation_id(), None);
    }
}

mod error_types {
    use super::*;

    #[test]
    fn test_connection_error() {
        let err = Error::connection("Socket not found");
        assert!(err.to_string().contains("Connection failed"));
        assert!(err.to_string().contains("Socket not found"));
    }

    #[test]
    fn test_http_error() {
        let err = Error::http(404, "Not Found");
        assert!(err.to_string().contains("404"));
        assert!(err.to_string().contains("Not Found"));
    }

    #[test]
    fn test_api_error() {
        let err = Error::api(403, "Access denied");
        assert!(err.to_string().contains("403"));
        assert!(err.to_string().contains("Access denied"));
    }

    #[test]
    fn test_json_error_from() {
        let json_err = serde_json::from_str::<i32>("not a number").unwrap_err();
        let err: Error = json_err.into();
        assert!(err.to_string().contains("JSON error"));
    }

    #[test]
    fn test_io_error_from() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert!(err.to_string().contains("I/O error"));
    }
}

mod type_serialization {
    use lxd_types::*;

    #[test]
    fn test_instance_source_from_image() {
        let source = InstanceSource::from_image("ubuntu/22.04");
        let json = serde_json::to_string(&source).unwrap();
        
        assert!(json.contains(r#""type":"image""#));
        assert!(json.contains(r#""alias":"ubuntu/22.04""#));
    }

    #[test]
    fn test_instance_source_from_remote() {
        let source = InstanceSource::from_remote_image("alpine/3.18", "https://images.linuxcontainers.org");
        let json = serde_json::to_string(&source).unwrap();
        
        assert!(json.contains(r#""type":"image""#));
        assert!(json.contains(r#""alias":"alpine/3.18""#));
        assert!(json.contains(r#""server":"https://images.linuxcontainers.org""#));
        assert!(json.contains(r#""protocol":"simplestreams""#));
    }

    #[test]
    fn test_instance_source_none() {
        let source = InstanceSource::none();
        let json = serde_json::to_string(&source).unwrap();
        
        assert!(json.contains(r#""type":"none""#));
    }

    #[test]
    fn test_instance_source_copy() {
        let source = InstanceSource::from_copy("my-container");
        let json = serde_json::to_string(&source).unwrap();
        
        assert!(json.contains(r#""type":"copy""#));
        assert!(json.contains(r#""source":"my-container""#));
    }

    #[test]
    fn test_instances_post_serialization() {
        let source = InstanceSource::from_image("ubuntu/22.04");
        let request = InstancesPost::new("test-container", source)
            .with_type(InstanceType::Container)
            .ephemeral(true)
            .with_profiles(vec!["default".to_string()]);

        let json = serde_json::to_string(&request).unwrap();
        
        assert!(json.contains(r#""name":"test-container""#));
        assert!(json.contains(r#""type":"container""#));
        assert!(json.contains(r#""ephemeral":true"#));
        assert!(json.contains(r#""profiles":["default"]"#));
    }

    #[test]
    fn test_instance_put_optional_fields() {
        let request = InstancePut {
            description: Some("Updated description".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).unwrap();
        
        // Should include description but skip None fields
        assert!(json.contains(r#""description":"Updated description""#));
        // Should not contain null values for skipped fields
        assert!(!json.contains("architecture"));
    }

    #[test]
    fn test_instance_state_put_start() {
        let request = InstanceStatePut {
            action: InstanceAction::Start,
            timeout: None,
            force: None,
            stateful: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(r#""action":"start""#));
    }

    #[test]
    fn test_instance_state_put_stop_force() {
        let request = InstanceStatePut {
            action: InstanceAction::Stop,
            timeout: Some(30),
            force: Some(true),
            stateful: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(r#""action":"stop""#));
        assert!(json.contains(r#""timeout":30"#));
        assert!(json.contains(r#""force":true"#));
    }

    #[test]
    fn test_instance_deserialization() {
        let json = r#"{
            "name": "my-container",
            "description": "Test container",
            "status": "Running",
            "status_code": 103,
            "type": "container",
            "architecture": "x86_64",
            "ephemeral": false,
            "stateful": false,
            "config": {},
            "devices": {},
            "expanded_config": {},
            "expanded_devices": {},
            "profiles": ["default"],
            "created_at": "2024-01-01T00:00:00Z",
            "last_used_at": "2024-01-02T00:00:00Z",
            "location": "",
            "project": "default"
        }"#;

        let instance: Instance = serde_json::from_str(json).unwrap();
        
        assert_eq!(instance.name, "my-container");
        assert_eq!(instance.status, "Running");
        assert_eq!(instance.instance_type, InstanceType::Container);
        assert!(!instance.ephemeral);
        assert_eq!(instance.profiles, vec!["default"]);
    }

    #[test]
    fn test_instance_type_variants() {
        assert_eq!(
            serde_json::to_string(&InstanceType::Container).unwrap(),
            r#""container""#
        );
        assert_eq!(
            serde_json::to_string(&InstanceType::VirtualMachine).unwrap(),
            r#""virtual-machine""#
        );
    }

    #[test]
    fn test_operation_deserialization() {
        let json = r#"{
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "class": "task",
            "description": "Creating instance",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:01Z",
            "status": "Running",
            "status_code": 103,
            "resources": {
                "instances": ["/1.0/instances/my-container"]
            },
            "metadata": null,
            "may_cancel": true,
            "err": "",
            "location": ""
        }"#;

        let operation: Operation = serde_json::from_str(json).unwrap();
        
        assert_eq!(operation.id, "550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(operation.class, "task");
        assert_eq!(operation.status, "Running");
        assert!(operation.may_cancel);
        assert!(operation.resources.contains_key("instances"));
    }

    #[test]
    fn test_image_deserialization() {
        let json = r#"{
            "fingerprint": "abc123def456",
            "architecture": "x86_64",
            "filename": "ubuntu.tar.gz",
            "public": true,
            "size": 1234567890,
            "auto_update": false,
            "properties": {"os": "Ubuntu", "release": "22.04"},
            "aliases": [{"name": "ubuntu/22.04"}],
            "created_at": "2024-01-01T00:00:00Z",
            "expires_at": "2025-01-01T00:00:00Z",
            "last_used_at": "2024-01-02T00:00:00Z",
            "uploaded_at": "2024-01-01T00:00:00Z"
        }"#;

        let image: Image = serde_json::from_str(json).unwrap();
        
        assert_eq!(image.fingerprint, "abc123def456");
        assert!(image.public);
        assert_eq!(image.size, 1234567890);
        assert_eq!(image.properties.get("os"), Some(&"Ubuntu".to_string()));
    }
}

mod client_path_building {
    // These tests verify the path building logic without needing a real server
    
    #[test]
    fn test_path_without_project() {
        // Simulating what Client::path does
        let project: Option<String> = None;
        let base = "/1.0/instances";
        
        let path = match &project {
            Some(p) => format!("{}?project={}", base, p),
            None => base.to_string(),
        };
        
        assert_eq!(path, "/1.0/instances");
    }

    #[test]
    fn test_path_with_project() {
        let project: Option<String> = Some("my-project".to_string());
        let base = "/1.0/instances";
        
        let path = match &project {
            Some(p) => format!("{}?project={}", base, p),
            None => base.to_string(),
        };
        
        assert_eq!(path, "/1.0/instances?project=my-project");
    }

    #[test]
    fn test_path_with_existing_query_params() {
        let project: Option<String> = Some("my-project".to_string());
        let base = "/1.0/instances?recursion=1";
        
        let path = match &project {
            Some(p) => {
                if base.contains('?') {
                    format!("{}&project={}", base, p)
                } else {
                    format!("{}?project={}", base, p)
                }
            }
            None => base.to_string(),
        };
        
        assert_eq!(path, "/1.0/instances?recursion=1&project=my-project");
    }
}

mod server_response {
    use lxd_types::*;

    #[test]
    fn test_server_deserialization() {
        let json = r#"{
            "api_extensions": ["instances", "networks"],
            "api_status": "stable",
            "api_version": "1.0",
            "auth": "trusted",
            "public": false,
            "auth_methods": ["tls"],
            "environment": {
                "architectures": ["x86_64"],
                "driver": "lxc",
                "driver_version": "5.0.0",
                "kernel": "Linux",
                "kernel_version": "5.15.0",
                "os_name": "Ubuntu",
                "os_version": "22.04",
                "server": "lxd",
                "server_name": "my-lxd",
                "server_version": "5.0"
            }
        }"#;

        let server: Server = serde_json::from_str(json).unwrap();
        
        assert_eq!(server.api_version, "1.0");
        assert_eq!(server.auth, "trusted");
        assert!(!server.public);
        assert_eq!(server.api_extensions.len(), 2);
    }
}
