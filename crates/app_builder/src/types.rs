#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub app_version: Option<String>,
}

#[derive(Debug)]
pub struct BuildResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug)]
pub enum BuildError {
    General(String),
}

impl std::error::Error for BuildError {}
