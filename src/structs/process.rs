use async_trait::async_trait;

use crate::traits::Component;

use super::{Event, Pool, Storage, Subscriptions};

/// single runnable process
pub struct Process {
    /// name of the current process
    label: String,

    /// unique identifier of the current process
    discrim: Vec<u32>,

    /// data storage for self
    pool: Pool,

    /// shared storage folder for self
    storage: Storage,

    /// subscribed events to be passed to process
    subscriptions: Subscriptions,
}

#[async_trait]
impl Component for Process {
    fn label(&self) -> &str {
        &self.label
    }

    fn discrim(&self) -> &Vec<u32> {
        &self.discrim
    }

    fn pool(&self) -> &Pool {
        &self.pool
    }

    fn storage(&self) -> &Storage {
        &self.storage
    }

    async fn pass(&mut self, event: &Event) -> bool {
        todo!()
    }
}
