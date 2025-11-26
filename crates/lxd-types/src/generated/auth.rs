//!//! Auth types for LXD API
#![allow(clippy::derive_partial_eq_without_eq)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[allow(unused_imports)]
use super::*;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthGroup {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///Description is a short description of the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Identities is a map of authentication method to slice of identity identifiers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identities: Option<BTreeMap<String, Vec<String>>>,
    /**IdentityProviderGroups are a list of groups from the IdP whose mapping
includes this group.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_provider_groups: Option<Vec<String>>,
    ///Name is the name of the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Permissions are a list of permissions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthGroupPost {
    ///Name is the name of the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthGroupPut {
    ///Description is a short description of the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Permissions are a list of permissions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthGroupsPost {
    ///Description is a short description of the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name is the name of the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Permissions are a list of permissions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    /**AuthenticationMethod is the authentication method that the identity
authenticates to LXD with.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<String>,
    ///Groups is the list of groups for which the identity is a member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    ///Identifier is a unique identifier for the identity (e.g. certificate fingerprint or email for OIDC).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /**Name is the Name claim of the identity if authenticated via OIDC, or the name
of the certificate if authenticated with TLS.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**TLSCertificate is a PEM encoded x509 certificate. This is only set if the AuthenticationMethod is AuthenticationMethodTLS.

API extension: access_management_tls.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls_certificate: Option<String>,
    ///Type is the type of identity.
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
/**IdentityBearerToken contains a token issued for an identity whose authentication method is
api.AuthenticationMethodBearer.*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityBearerToken {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityBearerTokenPost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
}
///These fields can only be evaluated for the currently authenticated identity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityInfo {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    /**AuthenticationMethod is the authentication method that the identity
authenticates to LXD with.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<String>,
    /**Effective groups is the combined and deduplicated list of LXD groups that the identity is a direct member of, and
the LXD groups that the identity is an effective member of via identity provider group mappings.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_groups: Option<Vec<String>>,
    /**Effective permissions is the combined and deduplicated list of permissions that the identity has by virtue of
direct membership to a LXD group, or effective membership of a LXD group via identity provider group mappings.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_permissions: Option<Vec<Permission>>,
    /**FineGrained is a boolean indicating whether the identity is fine-grained,
meaning that permissions are managed via group membership.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fine_grained: Option<bool>,
    ///Groups is the list of groups for which the identity is a member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    ///Identifier is a unique identifier for the identity (e.g. certificate fingerprint or email for OIDC).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /**Name is the Name claim of the identity if authenticated via OIDC, or the name
of the certificate if authenticated with TLS.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**TLSCertificate is a PEM encoded x509 certificate. This is only set if the AuthenticationMethod is AuthenticationMethodTLS.

API extension: access_management_tls.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls_certificate: Option<String>,
    ///Type is the type of identity.
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderGroup {
    ///AccessEntitlements represents the entitlements that are granted to the requesting user on the attached entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_entitlements: Option<Vec<String>>,
    ///Groups are the groups the IdP group resolves to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    ///Name is the name of the IdP group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderGroupPost {
    ///Name is the name of the IdP group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderGroupPut {
    ///Groups are the groups the IdP group resolves to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderGroupsPost {
    ///Groups are the groups the IdP group resolves to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    ///Name is the name of the IdP group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityPut {
    ///Groups is the list of groups for which the identity is a member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /**TLSCertificate is a base64 encoded x509 certificate. This can only be set if the authentication method of the identity is AuthenticationMethodTLS.

API extension: access_management_tls.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls_certificate: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Permission {
    ///Entitlement is the entitlement define for the entity type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitlement: Option<String>,
    ///EntityType is the string representation of the entity type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    ///EntityReference is the URL of the entity that the permission applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionInfo {
    ///Entitlement is the entitlement define for the entity type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitlement: Option<String>,
    ///EntityType is the string representation of the entity type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    ///Groups is a list of groups that have the Entitlement on the Entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    ///EntityReference is the URL of the entity that the permission applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
