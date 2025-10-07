use crate::traits::AppPublisher;
use crate::types::{PublishConfig, PublishError, PublishResult};

pub struct PlayStorePublisher;

impl AppPublisher for PlayStorePublisher {
    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn is_supported_on_current_platform(&self) -> bool {
        todo!()
    }

    fn publish(&self, config: PublishConfig) -> Result<PublishResult, PublishError> {
        todo!()
    }
}
