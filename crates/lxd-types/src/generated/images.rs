//!//! Images types for LXD API
#![allow(clippy::derive_partial_eq_without_eq)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
///Image represents a LXD image
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///List of aliases
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<ImageAlias>>,
    ///Architecture
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    ///Whether the image should auto-update when a new build is available
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<bool>,
    ///Whether the image is an automatically cached remote image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cached: Option<bool>,
    ///When the image was originally created
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    ///When the image becomes obsolete
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    ///Original filename
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    ///Full SHA-256 fingerprint
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    ///Last time the image was used
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    ///List of profiles to use when creating from this image (if none provided by user)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    ///Project name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///Descriptive properties
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, String>>,
    ///Whether the image is available to unauthenticated users
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    ///Size of the image in bytes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    ///Type of image (container or virtual-machine)
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_source: Option<ImageSource>,
    ///When the image was added to this LXD server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<String>,
}
///ImageAlias represents an alias from the alias list of a LXD image
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageAlias {
    ///Description of the alias
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name of the alias
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
///ImageAliasesEntry represents a LXD image alias
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageAliasesEntry {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///Alias description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Alias name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Target fingerprint for the alias
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    ///Alias type (container or virtual-machine)
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
///ImageAliasesEntryPost represents the required fields to rename a LXD image alias
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageAliasesEntryPost {
    ///Alias name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
///ImageAliasesEntryPut represents the modifiable fields of a LXD image alias
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageAliasesEntryPut {
    ///Alias description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Target fingerprint for the alias
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
///ImageAliasesPost represents a new LXD image alias
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageAliasesPost {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///Alias description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Alias name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Target fingerprint for the alias
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    ///Alias type (container or virtual-machine)
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
///ImageExportPost represents the fields required to export a LXD image
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageExportPost {
    ///List of aliases to set on the image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<ImageAlias>>,
    ///Remote server certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    ///List of profiles to use
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    ///Project name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///Image receive secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    ///Target server URL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
///ImageMetadata represents LXD image metadata (used in image tarball)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageMetadata {
    ///Architecture name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    ///Image creation data (as UNIX epoch)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<i64>,
    ///Image expiry data (as UNIX epoch)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<i64>,
    ///Descriptive properties
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, String>>,
    ///Template for files in the image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<BTreeMap<String, ImageMetadataTemplate>>,
}
///ImageMetadataTemplate represents a template entry in image metadata (used in image tarball)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageMetadataTemplate {
    ///Whether to trigger only if the file is missing
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_only: Option<bool>,
    ///Key/value properties to pass to the template
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, String>>,
    ///The template itself as a valid pongo2 template
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    ///When to trigger the template (create, copy or start)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<Vec<String>>,
}
///ImagePut represents the modifiable fields of a LXD image
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImagePut {
    ///Whether the image should auto-update when a new build is available
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<bool>,
    ///When the image becomes obsolete
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    ///List of profiles to use when creating from this image (if none provided by user)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    ///Descriptive properties
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, String>>,
    ///Whether the image is available to unauthenticated users
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
}
///ImageSource represents the source of a LXD image
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageSource {
    ///Source alias to download from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    ///Source server certificate (if not trusted by system CA)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    ///Type of image (container or virtual-machine)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
    ///Source server protocol
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    ///URL of the source server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}
///ImagesPost represents the fields available for a new LXD image
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImagesPost {
    ///Aliases to add to the image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<ImageAlias>>,
    ///Whether the image should auto-update when a new build is available
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<bool>,
    ///Compression algorithm to use when turning an instance into an image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression_algorithm: Option<String>,
    ///When the image becomes obsolete
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    ///Original filename of the image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    ///List of profiles to use when creating from this image (if none provided by user)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    ///Descriptive properties
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, String>>,
    ///Whether the image is available to unauthenticated users
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<ImagesPostSource>,
}
///ImagesPostSource represents the source of a new LXD image
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImagesPostSource {
    ///Source alias to download from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    ///Source server certificate (if not trusted by system CA)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    ///Source image fingerprint (for type "image")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    ///Type of image (container or virtual-machine)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
    ///Transfer mode (push or pull)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    ///Instance name (for type "instance" or "snapshot")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Source project name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    ///Source server protocol
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    ///Source image server secret token (when downloading private images)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    ///URL of the source server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    ///Type of image source (instance, snapshot, image or url)
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    ///Source URL (for type "url")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
