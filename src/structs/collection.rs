use std::{collections::HashMap, sync::Arc};

use tokio::{runtime, sync::Mutex};

use crate::traits::Component;

/// a collection of component items
pub struct Collection<T: Component> {
    items: HashMap<Vec<u32>, Arc<Mutex<T>>>,
}

impl<T: Component> Collection<T> {
    /// return all items matching label
    pub async fn find_all_by_label(&self, label: &str) -> Vec<Arc<Mutex<T>>> {
        self.items
            .iter()
            .filter_map(|(_, value)| {
                if label
                    == runtime::Builder::new_current_thread()
                        .build()
                        .unwrap()
                        .block_on(value.lock())
                        .label()
                {
                    Some(value.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// return max one element with matching discriminator
    pub fn find_by_discrim(&self, discrim: &Vec<u32>) -> Option<Arc<Mutex<T>>> {
        self.items.get(discrim).map(Arc::clone)
    }

    /// check if the item is in collection
    pub fn contains(&self, discrim: &Vec<u32>) -> bool {
        self.items.contains_key(discrim)
    }

    /// insert an item and return handle to it
    pub fn insert(&mut self, item: T) -> Option<Arc<Mutex<T>>> {
        self.contains(&item.discrim()).then(move || {
            let discrim = item.discrim().to_vec();
            let item = Arc::new(Mutex::new(item));
            self.items.insert(discrim, item.clone());
            item
        })
    }

    /// removes an item by discrim
    pub fn remove(&mut self, discrim: &Vec<u32>) -> bool {
        self.items.remove(discrim).is_some()
    }
}

impl<T: Component> Default for Collection<T> {
    fn default() -> Self {
        Self {
            items: HashMap::default(),
        }
    }
}
