//! Image types for LXD API

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Image represents an LXD image
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    /// Image fingerprint (SHA256)
    pub fingerprint: String,

    /// Image filename
    #[serde(default)]
    pub filename: String,

    /// Image size in bytes
    #[serde(default)]
    pub size: i64,

    /// Architecture
    pub architecture: String,

    /// Whether the image is public
    #[serde(default)]
    pub public: bool,

    /// Whether the image is cached
    #[serde(default)]
    pub cached: bool,

    /// Auto-update flag
    #[serde(default)]
    pub auto_update: bool,

    /// Image properties
    #[serde(default)]
    pub properties: BTreeMap<String, String>,

    /// Image aliases
    #[serde(default)]
    pub aliases: Vec<ImageAlias>,

    /// Creation timestamp
    #[serde(default)]
    pub created_at: String,

    /// Expiry timestamp
    #[serde(default)]
    pub expires_at: String,

    /// Last used timestamp
    #[serde(default)]
    pub last_used_at: String,

    /// Upload timestamp
    #[serde(default)]
    pub uploaded_at: String,

    /// Update source
    #[serde(default)]
    pub update_source: Option<ImageSource>,

    /// Image type
    #[serde(rename = "type", default)]
    pub image_type: String,
}

/// Image alias
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageAlias {
    /// Alias name
    pub name: String,

    /// Alias description
    #[serde(default)]
    pub description: String,
}

/// Image source for updates
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageSource {
    /// Server URL
    #[serde(default)]
    pub server: String,

    /// Protocol (simplestreams or lxd)
    #[serde(default)]
    pub protocol: String,

    /// Alias
    #[serde(default)]
    pub alias: String,

    /// Certificate
    #[serde(default)]
    pub certificate: String,
}

/// Request to create/import an image
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImagesPost {
    /// Image aliases to create
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<ImageAlias>>,

    /// Image properties
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, String>>,

    /// Whether the image is public
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,

    /// Auto-update flag
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<bool>,

    /// Image source
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<ImagesPostSource>,
}

/// Image source for importing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImagesPostSource {
    /// Source type
    #[serde(rename = "type")]
    pub source_type: String,

    /// Source mode
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Server URL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    /// Protocol
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// Secret for private images
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// Certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,

    /// Alias
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// Fingerprint
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Image properties
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, String>>,
}

impl ImagesPostSource {
    /// Create a source for downloading from a remote server
    pub fn from_remote(server: impl Into<String>, alias: impl Into<String>) -> Self {
        Self {
            source_type: "image".to_string(),
            mode: Some("pull".to_string()),
            server: Some(server.into()),
            protocol: Some("simplestreams".to_string()),
            alias: Some(alias.into()),
            secret: None,
            certificate: None,
            fingerprint: None,
            properties: None,
        }
    }
}

/// Request to update an image
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ImagePut {
    /// Image properties
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, String>>,

    /// Whether the image is public
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,

    /// Auto-update flag
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<bool>,

    /// Expiry timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}
