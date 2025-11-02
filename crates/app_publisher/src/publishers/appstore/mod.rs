use crate::traits::AppPublisher;
use crate::types::{PublishConfig, PublishError, PublishResult};

pub struct AppStorePublisher;

impl AppPublisher for AppStorePublisher {
    fn new() -> Self {
        Self
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn is_supported_on_current_platform(&self) -> bool {
        todo!()
    }

    fn perform_publish(&self, config: &PublishConfig) -> Result<PublishResult, PublishError> {
        todo!()
    }
}
