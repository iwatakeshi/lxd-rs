//! Swagger 2.0 specification parser

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Root OpenAPI/Swagger specification
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenApiSpec {
    /// OpenAPI version (3.x) - optional for LXD's spec
    #[serde(default)]
    pub openapi: String,
    /// Swagger version (2.x) - optional
    #[serde(default)]
    pub swagger: String,
    /// API info - optional
    #[serde(default)]
    pub info: Option<Info>,
    /// API paths/endpoints
    #[serde(default)]
    pub paths: BTreeMap<String, PathItem>,
    /// Type definitions (Swagger 2.0 style)
    #[serde(default)]
    pub definitions: BTreeMap<String, Schema>,
    /// Components (OpenAPI 3.0 style)
    #[serde(default)]
    pub components: Option<Components>,
}

/// API Info
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Info {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Components {
    #[serde(default)]
    pub schemas: BTreeMap<String, Schema>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PathItem {
    #[serde(default)]
    pub get: Option<Operation>,
    #[serde(default)]
    pub post: Option<Operation>,
    #[serde(default)]
    pub put: Option<Operation>,
    #[serde(default)]
    pub patch: Option<Operation>,
    #[serde(default)]
    pub delete: Option<Operation>,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    #[serde(default)]
    pub operation_id: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
    #[serde(default)]
    pub request_body: Option<RequestBody>,
    #[serde(default)]
    pub responses: BTreeMap<String, Response>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub location: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub schema: Option<Schema>,
    #[serde(rename = "type", default)]
    pub param_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestBody {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub content: BTreeMap<String, MediaType>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MediaType {
    #[serde(default)]
    pub schema: Option<Schema>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub content: BTreeMap<String, MediaType>,
    #[serde(default)]
    pub schema: Option<Schema>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "type", default)]
    pub schema_type: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub properties: BTreeMap<String, Schema>,
    #[serde(default)]
    pub items: Option<Box<Schema>>,
    #[serde(rename = "$ref", default)]
    pub reference: Option<String>,
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(rename = "enum", default)]
    pub enum_values: Vec<serde_yaml::Value>,
    #[serde(default)]
    pub additional_properties: Option<Box<Schema>>,
    #[serde(default)]
    pub all_of: Vec<Schema>,
    #[serde(default)]
    pub one_of: Vec<Schema>,
    #[serde(default)]
    pub any_of: Vec<Schema>,
    #[serde(default)]
    pub default: Option<serde_yaml::Value>,
    #[serde(default)]
    pub example: Option<serde_yaml::Value>,
    #[serde(rename = "x-go-name", default)]
    pub x_go_name: Option<String>,
}

impl Schema {
    /// Check if this is a reference to another schema
    #[allow(dead_code)]
    pub fn is_ref(&self) -> bool {
        self.reference.is_some()
    }

    /// Get the referenced type name
    pub fn ref_name(&self) -> Option<&str> {
        self.reference.as_ref().and_then(|r| {
            // Handle "#/definitions/TypeName" or "#/components/schemas/TypeName"
            r.rsplit('/').next()
        })
    }
}

/// Parse an OpenAPI YAML specification file
pub fn parse_openapi(path: &Path) -> anyhow::Result<OpenApiSpec> {
    let content = fs::read_to_string(path)?;
    let spec: OpenApiSpec = serde_yaml::from_str(&content)?;
    Ok(spec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref_name() {
        let schema = Schema {
            reference: Some("#/definitions/Instance".to_string()),
            ..Default::default()
        };
        assert_eq!(schema.ref_name(), Some("Instance"));
    }
}
