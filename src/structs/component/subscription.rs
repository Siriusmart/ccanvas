use serde::{Deserialize, Serialize};

/// a single subscription item, such as a key press event
#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub enum Subscription {
    /// subscribes to all key press events
    #[serde(rename = "all key presses")]
    AllKeyPresses,
}
