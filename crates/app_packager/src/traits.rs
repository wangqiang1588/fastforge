use crate::types::{PackageConfig, PackageError, PackageResult};

pub trait AppPackager {
    /// Get the name/identifier of this packager.
    fn name(&self) -> &str;

    /// Check if this packager is supported on the current platform.
    fn is_supported_on_current_platform(&self) -> bool;

    /// Package the application using the provided configuration.
    fn package(&self, config: PackageConfig) -> Result<PackageResult, PackageError>;
}
