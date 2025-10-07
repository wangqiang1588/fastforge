use crate::traits::AppAnalyzer;
use crate::types::{AnalyzeConfig, AnalyzeError, AnalyzeResult};

pub struct MacOSDmgAnalyzer;

impl AppAnalyzer for MacOSDmgAnalyzer {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn is_supported_on_current_platform(&self) -> bool {
        todo!()
    }

    fn perform_analyze(&self, config: &AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError> {
        todo!()
    }
}
