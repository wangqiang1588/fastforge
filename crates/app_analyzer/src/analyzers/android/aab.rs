use crate::traits::AppAnalyzer;
use crate::types::{AnalyzeConfig, AnalyzeError, AnalyzeResult};

pub struct AndroidAabAnalyzer;

impl AppAnalyzer for AndroidAabAnalyzer {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn is_supported_on_current_platform(&self) -> bool {
        todo!()
    }

    fn analyze(&self, config: AnalyzeConfig) -> Result<AnalyzeResult, AnalyzeError> {
        todo!()
    }
}
