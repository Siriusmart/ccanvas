use std::collections::HashSet;

use super::Subscription;

/// multiple subscriptions that a process can have
pub type Subscriptions = HashSet<Subscription>;
