use crate::types::{AnalyzeConfig, AnalyzeError, AnalyzeResult};

pub trait AppAnalyzer {
    /// Get the name/identifier of this analyzer.
    fn name(&self) -> &str;

    /// Check if this analyzer is supported on the current platform.
    fn is_supported_on_current_platform(&self) -> bool;

    /// Analyze the application using the provided configuration.
    fn analyze(&self, config: AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError>;
}
