use serde::Serialize;

#[derive(Serialize, Clone, PartialEq, Debug)]
pub enum ResponseSuccess {
    #[serde(rename = "subscribe added")]
    SubscribeAdded,

    #[serde(rename = "listener set")]
    ListenerSet,
}
