use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use super::{Process, Subscription};

pub struct PassItem {
    priority: u16,
    process: Arc<Mutex<Process>>,
}

pub struct Passes {
    subscriptions: HashMap<Subscription, Vec<PassItem>>,
}
