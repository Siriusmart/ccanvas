use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use super::{Process, Subscription};

/// a single subscription item
pub struct PassItem {
    /// 0 is highest
    /// None is lowest
    /// if there are clashes, first entry will revieve signal first
    priority: Option<u16>,
    /// pointer to process wrapper
    process: Arc<Mutex<Process>>,
}

/// stores which subspaces/subprocesses have subscribed to events
/// and pass events only to them in order of priority
pub struct Passes {
    subscriptions: HashMap<Subscription, Vec<PassItem>>,
}
