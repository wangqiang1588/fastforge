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
    pub data: serde_json::Value,
}

impl AnalyzeResult {
    pub fn new(success: bool, data: serde_json::Value) -> Self {
        Self { success, data }
    }
}

#[derive(Debug)]
pub enum AnalyzeError {
    General(String),
}

impl AnalyzeError {
    pub fn new(message: &str) -> Self {
        Self::General(message.to_string())
    }
}

impl std::fmt::Display for AnalyzeError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalyzeError::General(msg) => write!(_f, "Analyze error: {}", msg),
        }
    }
}

impl std::error::Error for AnalyzeError {}
