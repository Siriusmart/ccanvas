use async_trait::async_trait;

use crate::structs::{Event, Pool, Storage, Discriminator};

#[async_trait]
/// a unit of "something"
pub trait Component {
    /// unique identifier of what it is
    fn label(&self) -> &str;

    /// unique identifier which one it is
    fn discrim(&self) -> &Discriminator;

    /// pool of data
    fn pool(&self) -> &Pool;

    /// folder for shared storage
    fn storage(&self) -> &Storage;

    /// pass an event into a component
    /// returns true to pass event to next component, false otherwise
    async fn pass(&mut self, event: &Event) -> bool;
}
