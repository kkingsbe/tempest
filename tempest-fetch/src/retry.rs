//! Retry logic with exponential backoff for S3 requests.

use std::time::Duration;

use tracing::{info, warn};

use crate::error::FetchError;

/// Configuration for retry behavior.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts.
    pub max_retries: u32,
    /// Base delay for exponential backoff.
    pub base_delay: Duration,
    /// Maximum delay between retries.
    pub max_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
        }
    }
}

/// Trait for determining if an error should trigger a retry.
pub trait RetryableError {
    /// Returns true if the error is transient and should be retried.
    fn is_retryable(&self) -> bool;
}

impl RetryableError for FetchError {
    fn is_retryable(&self) -> bool {
        match self {
            // Retry on network issues, timeouts, and rate limiting
            FetchError::Network(_) => true,
            FetchError::Timeout(_) => true,
            FetchError::TooManyRequests(_) => true,
            // Also retry on generic HTTP errors (likely 5xx server errors)
            FetchError::Http(_) => true,
            // Do not retry on S3 not found (404) - resource doesn't exist
            FetchError::S3NotFound(_) => false,
            // Do not retry on cache I/O errors - likely a persistent issue
            FetchError::CacheIo(_) => false,
            // Do not retry on cache errors
            FetchError::CacheError(_) => false,
            // Do not retry on I/O errors - likely a persistent issue
            FetchError::Io(_) => false,
            // Do not retry on S3 errors
            FetchError::S3Error(_) => false,
            // Do not retry on not found
            FetchError::NotFound(_) => false,
            // Do not retry on internal errors
            FetchError::Internal(_) => false,
        }
    }
}

/// Execute an async operation with exponential backoff retry logic.
///
/// # Arguments
/// * `config` - Retry configuration
/// * `operation` - Async operation to execute
///
/// # Returns
/// * `Ok(T)` - If the operation succeeds
/// * `Err(E)` - If all retry attempts fail
///
/// # Example
/// ```ignore
/// let result = with_retry(RetryConfig::default(), async {
///     fetch_s3_data("bucket", "key").await
/// }).await;
/// ```
pub async fn with_retry<T, E, F, Fut>(config: RetryConfig, operation: F) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: RetryableError + std::fmt::Debug + Clone,
{
    let mut last_error: Option<E> = None;

    for attempt in 0..config.max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                last_error = Some(error.clone());

                // Check if we should retry
                if !error.is_retryable() {
                    info!("Non-retryable error encountered, not retrying: {:?}", error);
                    return Err(error);
                }

                // If this is not the last attempt, calculate delay and wait
                if attempt < config.max_retries - 1 {
                    // Exponential backoff: base_delay * 2^attempt
                    let delay_secs = 2u64.pow(attempt) * config.base_delay.as_secs();
                    let delay = Duration::from_secs(delay_secs.min(config.max_delay.as_secs()));

                    info!(
                        "Retry attempt {}/{} after {:?} - error: {:?}",
                        attempt + 1,
                        config.max_retries,
                        delay,
                        error
                    );

                    tokio::time::sleep(delay).await;
                } else {
                    warn!(
                        "All {} retry attempts exhausted - final error: {:?}",
                        config.max_retries, error
                    );
                }
            }
        }
    }

    // All retries exhausted
    Err(last_error.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retry_success_first_attempt() {
        let config = RetryConfig::default();
        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));

        let result = with_retry(config, || {
            let call_count = call_count.clone();
            async move {
                call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Ok::<i32, FetchError>(42)
            }
        })
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_retry_success_after_failures() {
        let config = RetryConfig {
            max_retries: 3,
            base_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(100),
        };
        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));

        let result = with_retry(config, || {
            let call_count = call_count.clone();
            async move {
                let count = call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if count < 2 {
                    Err(FetchError::Network("temporary failure".to_string()))
                } else {
                    Ok(42)
                }
            }
        })
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_all_failures() {
        let config = RetryConfig {
            max_retries: 3,
            base_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(100),
        };
        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));

        let result = with_retry(config, || {
            let call_count = call_count.clone();
            async move {
                call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Err::<i32, FetchError>(FetchError::Network("persistent failure".to_string()))
            }
        })
        .await;

        assert!(result.is_err());
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_non_retryable_error() {
        let config = RetryConfig {
            max_retries: 3,
            base_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(100),
        };
        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));

        let result = with_retry(config, || {
            let call_count = call_count.clone();
            async move {
                call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Err::<i32, FetchError>(FetchError::NotFound("not found".to_string()))
            }
        })
        .await;

        assert!(result.is_err());
        // Should only attempt once for non-retryable errors
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[test]
    fn test_retryable_error_network() {
        let error = FetchError::Network("connection refused".to_string());
        assert!(error.is_retryable());
    }

    #[test]
    fn test_retryable_error_timeout() {
        let error = FetchError::Timeout("request timed out".to_string());
        assert!(error.is_retryable());
    }

    #[test]
    fn test_retryable_error_rate_limited() {
        let error = FetchError::TooManyRequests("rate limited".to_string());
        assert!(error.is_retryable());
    }

    #[test]
    fn test_retryable_error_not_found() {
        let error = FetchError::S3NotFound("not found".to_string());
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_retryable_error_cache_io() {
        let error = FetchError::CacheIo("disk full".to_string());
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_retryable_error_internal() {
        let error = FetchError::Internal("bug".to_string());
        assert!(!error.is_retryable());
    }
}
