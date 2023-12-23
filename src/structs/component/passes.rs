use std::collections::{hash_map::Entry, HashMap};

use super::{Discriminator, Subscription};

/// a single subscription item
#[derive(Eq, Clone)]
pub struct PassItem {
    /// 0 is highest
    /// None is lowest
    /// if there are clashes, first entry will revieve signal first
    priority: Option<u32>,
    /// pointer to process wrapper
    // process: Arc<Mutex<Process>>,
    /// discrim of process
    discrim: Discriminator,
}

impl PassItem {
    pub fn discrim(&self) -> &Discriminator {
        &self.discrim
    }

    pub fn new(discrim: Discriminator, priority: Option<u32>) -> Self {
        Self { priority, discrim }
    }
}

impl PartialOrd for PassItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PassItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialEq for PassItem {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

/// stores which subspaces/subprocesses have subscribed to events
/// and pass events only to them in order of priority
#[derive(Default)]
pub struct Passes {
    subscriptions: HashMap<Subscription, Vec<PassItem>>,
}

impl Passes {
    /// add pass item
    pub fn subscribe(&mut self, subscription: Subscription, item: PassItem) {
        let items = self.subscriptions.entry(subscription).or_default();

        if let Some(index) = items.iter().position(|x| x.discrim() == item.discrim()) {
            items.remove(index);
        }

        // put it in the right place
        // according to priority
        for i in 0..items.len() {
            if items[i] > item {
                items.insert(i, item);
                return;
            }
        }

        items.push(item);
    }

    /// remove pass item
    pub fn unsubscribe(&mut self, subscription: Subscription, discrim: &Discriminator) -> bool {
        let mut items = if let Entry::Occupied(items) = self.subscriptions.entry(subscription) {
            items
        } else {
            return false;
        };

        if let Some(index) = items.get().iter().position(|x| x.discrim() == discrim) {
            items.get_mut().remove(index);
            if items.get().is_empty() {
                items.remove_entry();
            }
            return true;
        }

        false
    }

    /// list subscribers of all the subscriptions specified
    /// sorted + no duplicates
    pub fn subscribers(&self, subscription: &[Subscription]) -> Vec<PassItem> {
        let default = Vec::default(); // wow im so good at going around ownership checks
        let mut subscribers = subscription
            .iter()
            .flat_map(|sub| self.subscriptions.get(sub).unwrap_or(&default))
            .collect::<Vec<_>>();

        // they are now according to priority
        // but there may be duplicates
        subscribers.sort();

        // only take the highest priority
        // if a component has multiple subscriptions on this event
        let mut out = Vec::new();
        subscribers.into_iter().for_each(|sub| {
            if out.binary_search(sub).is_err() {
                // if its error
                // it means its not there
                // this works because it is sorted at all times
                out.push(sub.to_owned())
            }
        });

        out
    }
}