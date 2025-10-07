#[derive(Debug, Clone)]
pub struct PublishConfig {
    pub app_version: Option<String>,
}

#[derive(Debug)]
pub struct PublishResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug)]
pub enum PublishError {
    General(String),
}

impl std::fmt::Display for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for PublishError {}
