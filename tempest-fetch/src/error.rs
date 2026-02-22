//! Error types for tempest-fetch crate.

use thiserror::Error;

/// Errors that can occur during fetch operations.
#[derive(Debug, Clone, Error)]
pub enum FetchError {
    /// I/O error occurred.
    #[error("I/O error: {0}")]
    Io(String),

    /// Network-related error.
    #[error("Network error: {0}")]
    Network(String),

    /// Request timeout.
    #[error("Timeout: {0}")]
    Timeout(String),

    /// Rate limit exceeded (HTTP 429).
    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    /// S3 resource not found (HTTP 404).
    #[error("S3 not found: {0}")]
    S3NotFound(String),

    /// Cache I/O error.
    #[error("Cache I/O error: {0}")]
    CacheIo(String),

    /// HTTP error (non-2xx status code).
    #[error("HTTP error: {0}")]
    Http(String),

    /// Internal error (bugs, unexpected states).
    #[error("Internal error: {0}")]
    Internal(String),

    /// Generic cache error.
    #[error("Cache error: {0}")]
    CacheError(String),

    /// S3-related error.
    #[error("S3 error: {0}")]
    S3Error(String),

    /// Resource not found.
    #[error("Not found: {0}")]
    NotFound(String),
}

impl FetchError {
    /// Create a new I/O error.
    pub fn io(msg: impl Into<String>) -> Self {
        Self::Io(msg.into())
    }

    /// Create a new cache error with a message.
    pub fn cache(msg: impl Into<String>) -> Self {
        Self::CacheError(msg.into())
    }

    /// Create a new cache I/O error.
    pub fn cache_io(msg: impl Into<String>) -> Self {
        Self::CacheIo(msg.into())
    }

    /// Create a new not found error.
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    /// Create a new S3 error.
    pub fn s3(msg: impl Into<String>) -> Self {
        Self::S3Error(msg.into())
    }

    /// Create a new network error.
    pub fn network(msg: impl Into<String>) -> Self {
        Self::Network(msg.into())
    }

    /// Create a new timeout error.
    pub fn timeout(msg: impl Into<String>) -> Self {
        Self::Timeout(msg.into())
    }

    /// Create a new rate limit error.
    pub fn too_many_requests(msg: impl Into<String>) -> Self {
        Self::TooManyRequests(msg.into())
    }

    /// Create a new S3 not found error.
    pub fn s3_not_found(msg: impl Into<String>) -> Self {
        Self::S3NotFound(msg.into())
    }

    /// Create a new HTTP error.
    pub fn http(msg: impl Into<String>) -> Self {
        Self::Http(msg.into())
    }

    /// Create a new internal error.
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}

impl From<std::io::Error> for FetchError {
    fn from(err: std::io::Error) -> Self {
        FetchError::Io(err.to_string())
    }
}

impl From<reqwest::Error> for FetchError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            FetchError::Timeout(err.to_string())
        } else if err.is_connect() {
            FetchError::Network(err.to_string())
        } else if let Some(status) = err.status() {
            match status.as_u16() {
                404 => FetchError::S3NotFound(err.to_string()),
                429 => FetchError::TooManyRequests(err.to_string()),
                _ => FetchError::Http(err.to_string()),
            }
        } else {
            FetchError::Network(err.to_string())
        }
    }
}
