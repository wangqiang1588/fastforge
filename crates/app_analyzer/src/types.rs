#[derive(Debug, Clone)]
pub struct AnalyzeConfig {}

#[derive(Debug)]
pub struct AnalyzeResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug)]
pub enum AnalyzeError {
    General(String),
}

impl std::error::Error for AnalyzeError {}
