#[derive(Debug, Clone)]
pub struct PackageConfig {
    pub app_version: Option<String>,
}

#[derive(Debug)]
pub struct PackageResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug)]
pub enum PackageError {
    General(String),
}

impl std::error::Error for PackageError {}
