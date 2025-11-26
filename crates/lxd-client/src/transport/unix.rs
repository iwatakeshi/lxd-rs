//! Unix socket transport for LXD API

use crate::{Error, Result};
use crate::transport::Transport;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::client::legacy::Client;
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use serde::{de::DeserializeOwned, Serialize};
use std::path::{Path, PathBuf};

/// Unix socket transport for LXD API
pub struct UnixSocketTransport {
    socket_path: PathBuf,
    client: Client<UnixConnector, Full<Bytes>>,
}

impl UnixSocketTransport {
    /// Create a new Unix socket transport
    pub fn new(socket_path: impl AsRef<Path>) -> Result<Self> {
        let socket_path = socket_path.as_ref().to_path_buf();
        
        if !socket_path.exists() {
            return Err(Error::connection(format!(
                "Unix socket not found: {}",
                socket_path.display()
            )));
        }
        
        let client: Client<UnixConnector, Full<Bytes>> = 
            Client::unix();
        
        Ok(Self {
            socket_path,
            client,
        })
    }
    
    /// Build a request
    fn build_request(&self, method: Method, path: &str, body: Option<Bytes>) -> Result<Request<Full<Bytes>>> {
        let uri = Uri::new(&self.socket_path, path);
        
        let builder = Request::builder()
            .method(method)
            .uri(uri)
            .header("Host", "localhost")
            .header("Content-Type", "application/json");
        
        let body = body.unwrap_or_default();
        
        builder
            .body(Full::new(body))
            .map_err(|e| Error::request(e.to_string()))
    }
    
    /// Send a request and parse the response
    async fn send_request<T: DeserializeOwned>(&self, request: Request<Full<Bytes>>) -> Result<T> {
        let response: Response<Incoming> = self.client
            .request(request)
            .await
            .map_err(|e| Error::connection(e.to_string()))?;
        
        let status = response.status();
        let body = response
            .into_body()
            .collect()
            .await
            .map_err(|e| Error::request(e.to_string()))?
            .to_bytes();
        
        if !status.is_success() && status != StatusCode::ACCEPTED {
            // Try to parse error response
            if let Ok(error_resp) = serde_json::from_slice::<lxd_types::Response<()>>(&body) {
                return Err(Error::api(error_resp.error_code, error_resp.error));
            }
            return Err(Error::http(status.as_u16(), String::from_utf8_lossy(&body).to_string()));
        }
        
        serde_json::from_slice(&body).map_err(Error::Json)
    }
}

impl Transport for UnixSocketTransport {
    async fn get<T: DeserializeOwned + Send>(&self, path: &str) -> Result<T> {
        let request = self.build_request(Method::GET, path, None)?;
        self.send_request(request).await
    }
    
    async fn post<T: DeserializeOwned + Send, B: Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let body_bytes = serde_json::to_vec(body)?;
        let request = self.build_request(Method::POST, path, Some(Bytes::from(body_bytes)))?;
        self.send_request(request).await
    }
    
    async fn put<T: DeserializeOwned + Send, B: Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let body_bytes = serde_json::to_vec(body)?;
        let request = self.build_request(Method::PUT, path, Some(Bytes::from(body_bytes)))?;
        self.send_request(request).await
    }
    
    async fn patch<T: DeserializeOwned + Send, B: Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let body_bytes = serde_json::to_vec(body)?;
        let request = self.build_request(Method::PATCH, path, Some(Bytes::from(body_bytes)))?;
        self.send_request(request).await
    }
    
    async fn delete<T: DeserializeOwned + Send>(&self, path: &str) -> Result<T> {
        let request = self.build_request(Method::DELETE, path, None)?;
        self.send_request(request).await
    }
}
