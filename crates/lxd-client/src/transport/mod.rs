//! Transport layer for LXD API communication

mod unix;
mod https;

pub use unix::UnixSocketTransport;
pub use https::HttpsTransport;

use crate::Result;
use serde::{de::DeserializeOwned, Serialize};

/// Transport kind enum for runtime dispatch
pub enum TransportKind {
    /// Unix socket transport
    UnixSocket(UnixSocketTransport),
    /// HTTPS transport
    Https(HttpsTransport),
}

impl TransportKind {
    /// Perform a GET request
    pub async fn get<T: DeserializeOwned + Send>(&self, path: &str) -> Result<T> {
        match self {
            TransportKind::UnixSocket(t) => t.get(path).await,
            TransportKind::Https(t) => t.get(path).await,
        }
    }
    
    /// Perform a POST request
    pub async fn post<T: DeserializeOwned + Send, B: Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        match self {
            TransportKind::UnixSocket(t) => t.post(path, body).await,
            TransportKind::Https(t) => t.post(path, body).await,
        }
    }
    
    /// Perform a PUT request
    pub async fn put<T: DeserializeOwned + Send, B: Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        match self {
            TransportKind::UnixSocket(t) => t.put(path, body).await,
            TransportKind::Https(t) => t.put(path, body).await,
        }
    }
    
    /// Perform a PATCH request
    pub async fn patch<T: DeserializeOwned + Send, B: Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        match self {
            TransportKind::UnixSocket(t) => t.patch(path, body).await,
            TransportKind::Https(t) => t.patch(path, body).await,
        }
    }
    
    /// Perform a DELETE request
    pub async fn delete<T: DeserializeOwned + Send>(&self, path: &str) -> Result<T> {
        match self {
            TransportKind::UnixSocket(t) => t.delete(path).await,
            TransportKind::Https(t) => t.delete(path).await,
        }
    }
}

/// Transport trait for LXD API communication
pub trait Transport: Send + Sync {
    /// Perform a GET request
    fn get<T: DeserializeOwned + Send>(&self, path: &str) -> impl std::future::Future<Output = Result<T>> + Send;
    
    /// Perform a POST request
    fn post<T: DeserializeOwned + Send, B: Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> impl std::future::Future<Output = Result<T>> + Send;
    
    /// Perform a PUT request
    fn put<T: DeserializeOwned + Send, B: Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> impl std::future::Future<Output = Result<T>> + Send;
    
    /// Perform a PATCH request
    fn patch<T: DeserializeOwned + Send, B: Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> impl std::future::Future<Output = Result<T>> + Send;
    
    /// Perform a DELETE request
    fn delete<T: DeserializeOwned + Send>(&self, path: &str) -> impl std::future::Future<Output = Result<T>> + Send;
}
