use serde::Serialize;

use crate::structs::Discriminator;

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum ResponseSuccess {
    #[serde(rename = "subscribe added")]
    SubscribeAdded,

    #[serde(rename = "listener set")]
    ListenerSet,

    #[serde(rename = "dropped")]
    Dropped,

    #[serde(rename = "rendered")]
    Rendered,

    #[serde(rename = "spawned")]
    Spawned { discrim: Discriminator },
}
