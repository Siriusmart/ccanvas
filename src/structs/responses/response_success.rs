use serde::Serialize;

use crate::structs::Discriminator;

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum ResponseSuccess {
    /// subscription added
    #[serde(rename = "subscribe added")]
    SubscribeAdded,

    /// subscription removed
    #[serde(rename = "subscribe removed")]
    SubscribeRemoved,

    /// listener socket set
    #[serde(rename = "listener set")]
    ListenerSet,

    /// component dropped
    #[serde(rename = "dropped")]
    Dropped,

    /// render task completed
    #[serde(rename = "rendered")]
    Rendered,

    /// process spawned with discrim
    #[serde(rename = "spawned")]
    Spawned { discrim: Discriminator },

    /// message delivered ot target
    #[serde(rename = "message delivered")]
    MessageDelivered,

    /// space created with discrim
    #[serde(rename = "space created")]
    SpaceCreated { discrim: Discriminator },

    /// focus changed successfully
    #[serde(rename = "focus changed")]
    FocusChanged,
}
