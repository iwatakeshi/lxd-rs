//! Error types for the LXD client

use std::io;
use thiserror::Error;

/// LXD client error type
#[derive(Debug, Error)]
pub enum Error {
    /// Connection error
    #[error("Connection failed: {0}")]
    Connection(String),

    /// HTTP error
    #[error("HTTP error: {status} - {message}")]
    Http { status: u16, message: String },

    /// API error from LXD
    #[error("LXD API error: {code} - {message}")]
    Api { code: i64, message: String },

    /// Serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Request error
    #[error("Request error: {0}")]
    Request(String),

    /// Operation failed
    #[error("Operation failed: {0}")]
    OperationFailed(String),

    /// Timeout
    #[error("Operation timed out")]
    Timeout,

    /// TLS error
    #[error("TLS error: {0}")]
    Tls(String),

    /// Other error
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Create a connection error
    pub fn connection(msg: impl Into<String>) -> Self {
        Self::Connection(msg.into())
    }

    /// Create an HTTP error
    pub fn http(status: u16, msg: impl Into<String>) -> Self {
        Self::Http {
            status,
            message: msg.into(),
        }
    }

    /// Create an API error
    pub fn api(code: i64, msg: impl Into<String>) -> Self {
        Self::Api {
            code,
            message: msg.into(),
        }
    }

    /// Create a request error
    pub fn request(msg: impl Into<String>) -> Self {
        Self::Request(msg.into())
    }
}

/// Result type for LXD client operations
pub type Result<T> = std::result::Result<T, Error>;
