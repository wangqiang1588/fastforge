use crate::types::{PublishConfig, PublishError, PublishResult};

pub trait AppPublisher {
    /// Get the name/identifier of this publisher.
    fn name(&self) -> &str;

    /// Check if this publisher is supported on the current platform.
    fn is_supported_on_current_platform(&self) -> bool;

    /// Publish the application using the provided configuration.
    fn publish(&self, config: &PublishConfig) -> Result<PublishResult, PublishError>;
}
