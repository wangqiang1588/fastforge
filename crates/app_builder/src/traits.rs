use crate::types::{BuildConfig, BuildError, BuildResult};

pub trait AppBuilder {
    /// Get the name/identifier of this builder.
    fn name(&self) -> &str;

    /// Check if this builder is supported on the current platform.
    fn is_supported_on_current_platform(&self) -> bool;

    /// Build the application using the provided configuration.
    fn build(&self, config: BuildConfig) -> Result<BuildResult, BuildError>;
}
