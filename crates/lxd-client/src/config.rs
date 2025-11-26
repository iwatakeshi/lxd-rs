//! Client configuration options

use std::time::Duration;

/// Client configuration
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Request timeout
    pub timeout: Duration,

    /// Number of retries for transient failures
    pub retries: u32,

    /// Initial retry delay (doubles with each retry)
    pub retry_delay: Duration,

    /// Maximum retry delay
    pub max_retry_delay: Duration,

    /// Whether to retry on connection errors
    pub retry_on_connection_error: bool,

    /// Whether to retry on 5xx errors
    pub retry_on_server_error: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            retries: 3,
            retry_delay: Duration::from_millis(100),
            max_retry_delay: Duration::from_secs(5),
            retry_on_connection_error: true,
            retry_on_server_error: true,
        }
    }
}

impl ClientConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the number of retries
    pub fn with_retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    /// Set the initial retry delay
    pub fn with_retry_delay(mut self, delay: Duration) -> Self {
        self.retry_delay = delay;
        self
    }

    /// Set the maximum retry delay
    pub fn with_max_retry_delay(mut self, delay: Duration) -> Self {
        self.max_retry_delay = delay;
        self
    }

    /// Disable all retries
    pub fn no_retries(mut self) -> Self {
        self.retries = 0;
        self
    }

    /// Calculate the delay for a given retry attempt (exponential backoff)
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let delay = self.retry_delay.as_millis() as u64 * 2u64.pow(attempt);
        let max = self.max_retry_delay.as_millis() as u64;
        Duration::from_millis(delay.min(max))
    }
}

/// Builder for creating a configured client
pub struct ClientBuilder {
    config: ClientConfig,
    project: Option<String>,
}

impl ClientBuilder {
    /// Create a new client builder with default configuration
    pub fn new() -> Self {
        Self {
            config: ClientConfig::default(),
            project: None,
        }
    }

    /// Set the client configuration
    pub fn config(mut self, config: ClientConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Set the number of retries
    pub fn retries(mut self, retries: u32) -> Self {
        self.config.retries = retries;
        self
    }

    /// Set the project for all requests
    pub fn project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(project.into());
        self
    }

    /// Get the configuration
    pub fn get_config(&self) -> &ClientConfig {
        &self.config
    }

    /// Get the project
    pub fn get_project(&self) -> Option<&str> {
        self.project.as_deref()
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ClientConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.retries, 3);
        assert!(config.retry_on_connection_error);
    }

    #[test]
    fn test_exponential_backoff() {
        let config = ClientConfig::new()
            .with_retry_delay(Duration::from_millis(100))
            .with_max_retry_delay(Duration::from_secs(5));

        assert_eq!(config.delay_for_attempt(0), Duration::from_millis(100));
        assert_eq!(config.delay_for_attempt(1), Duration::from_millis(200));
        assert_eq!(config.delay_for_attempt(2), Duration::from_millis(400));
        assert_eq!(config.delay_for_attempt(3), Duration::from_millis(800));

        // Should cap at max
        assert_eq!(config.delay_for_attempt(10), Duration::from_secs(5));
    }

    #[test]
    fn test_builder() {
        let builder = ClientBuilder::new()
            .timeout(Duration::from_secs(60))
            .retries(5)
            .project("my-project");

        assert_eq!(builder.get_config().timeout, Duration::from_secs(60));
        assert_eq!(builder.get_config().retries, 5);
        assert_eq!(builder.get_project(), Some("my-project"));
    }
}
