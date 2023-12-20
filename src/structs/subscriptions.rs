use std::collections::HashSet;

use super::Subscription;

/// multiple subscriptions that a process can have
pub struct Subscriptions {
    subscriptions: HashSet<Subscription>,
}
