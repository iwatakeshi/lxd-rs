//! HTTPS transport for LXD API

use crate::transport::Transport;
use crate::{Error, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::Path;

/// HTTPS transport for remote LXD API
pub struct HttpsTransport {
    base_url: String,
    client: reqwest::Client,
}

impl HttpsTransport {
    /// Create a new HTTPS transport
    pub fn new(
        url: impl Into<String>,
        cert_path: impl AsRef<Path>,
        key_path: impl AsRef<Path>,
    ) -> Result<Self> {
        let base_url = url.into().trim_end_matches('/').to_string();

        // Read certificate and key
        let cert_pem = fs::read(cert_path.as_ref())
            .map_err(|e| Error::Tls(format!("Failed to read certificate: {}", e)))?;
        let key_pem = fs::read(key_path.as_ref())
            .map_err(|e| Error::Tls(format!("Failed to read key: {}", e)))?;

        // Create identity from cert + key
        let identity = reqwest::Identity::from_pem(&[cert_pem.clone(), key_pem].concat())
            .map_err(|e| Error::Tls(format!("Failed to create identity: {}", e)))?;

        // Build client with custom TLS config
        let client = reqwest::Client::builder()
            .identity(identity)
            .danger_accept_invalid_certs(true) // LXD uses self-signed certs
            .build()
            .map_err(|e| Error::Tls(format!("Failed to build client: {}", e)))?;

        Ok(Self { base_url, client })
    }

    /// Build full URL from path
    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Parse response and handle errors
    async fn parse_response<T: DeserializeOwned>(response: reqwest::Response) -> Result<T> {
        let status = response.status();
        let body = response
            .bytes()
            .await
            .map_err(|e| Error::request(e.to_string()))?;

        if !status.is_success() && status != reqwest::StatusCode::ACCEPTED {
            // Try to parse error response
            if let Ok(error_resp) = serde_json::from_slice::<lxd_types::Response<()>>(&body) {
                return Err(Error::api(error_resp.error_code, error_resp.error));
            }
            return Err(Error::http(
                status.as_u16(),
                String::from_utf8_lossy(&body).to_string(),
            ));
        }

        serde_json::from_slice(&body).map_err(Error::Json)
    }
}

impl Transport for HttpsTransport {
    async fn get<T: DeserializeOwned + Send>(&self, path: &str) -> Result<T> {
        let response = self
            .client
            .get(self.url(path))
            .send()
            .await
            .map_err(|e| Error::connection(e.to_string()))?;

        Self::parse_response(response).await
    }

    async fn post<T: DeserializeOwned + Send, B: Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let response = self
            .client
            .post(self.url(path))
            .json(body)
            .send()
            .await
            .map_err(|e| Error::connection(e.to_string()))?;

        Self::parse_response(response).await
    }

    async fn put<T: DeserializeOwned + Send, B: Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let response = self
            .client
            .put(self.url(path))
            .json(body)
            .send()
            .await
            .map_err(|e| Error::connection(e.to_string()))?;

        Self::parse_response(response).await
    }

    async fn patch<T: DeserializeOwned + Send, B: Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let response = self
            .client
            .patch(self.url(path))
            .json(body)
            .send()
            .await
            .map_err(|e| Error::connection(e.to_string()))?;

        Self::parse_response(response).await
    }

    async fn delete<T: DeserializeOwned + Send>(&self, path: &str) -> Result<T> {
        let response = self
            .client
            .delete(self.url(path))
            .send()
            .await
            .map_err(|e| Error::connection(e.to_string()))?;

        Self::parse_response(response).await
    }
}
