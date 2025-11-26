//!//! Certificates types for LXD API
#![allow(clippy::derive_partial_eq_without_eq)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
///Certificate represents a LXD certificate
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Certificate {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///The certificate itself, as PEM encoded X509 certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    ///SHA256 fingerprint of the certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    ///Name associated with the certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///List of allowed projects (applies when restricted)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
    ///Whether to limit the certificate to listed projects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    ///Usage type for the certificate
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateAddToken {
    ///The addresses of the server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    ///The name of the new client
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    ///The token's expiry date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    ///The fingerprint of the network certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    ///The random join secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    ///Type is an indicator for which API (certificates or identities) to send the token.
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
///CertificatePut represents the modifiable fields of a LXD certificate
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificatePut {
    ///The certificate itself, as PEM encoded X509 certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    ///Name associated with the certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///List of allowed projects (applies when restricted)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
    ///Whether to limit the certificate to listed projects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    ///Usage type for the certificate
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
///CertificatesPost represents the fields of a new LXD certificate
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificatesPost {
    ///The certificate itself, as base64 encoded X509 PEM certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    ///Name associated with the certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Server trust password (used to add an untrusted client, deprecated, use trust_token)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    ///List of allowed projects (applies when restricted)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
    ///Whether to limit the certificate to listed projects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    ///Whether to create a certificate add token
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<bool>,
    ///Trust token (used to add an untrusted client)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust_token: Option<String>,
    ///Usage type for the certificate
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
