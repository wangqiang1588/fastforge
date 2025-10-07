#[derive(Debug, Clone)]
pub struct AnalyzeConfig {
    pub path: String,
}

impl AnalyzeConfig {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

#[derive(Debug)]
pub struct AnalyzeResult {
    pub success: bool,
    pub message: String,
}

impl AnalyzeResult {
    pub fn new(success: bool, message: String) -> Self {
        Self { success, message }
    }
}

#[derive(Debug)]
pub enum AnalyzeError {
    General(String),
}

impl std::fmt::Display for AnalyzeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for AnalyzeError {}
