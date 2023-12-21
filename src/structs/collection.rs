use std::collections::HashMap;

use crate::traits::Component;

use super::Discriminator;

/// a collection of component items
pub struct Collection<T: Component> {
    // items: HashMap<Discriminator, Arc<Mutex<T>>>,
    items: HashMap<Discriminator, T>,
}

impl<T: Component> Collection<T> {
    /// return all items matching label
    // pub async fn find_all_by_label(&self, label: &str) -> Vec<Arc<Mutex<T>>> {
    //     self.items
    //         .iter()
    //         .filter_map(|(_, value)| {
    //             if label
    //                 == runtime::Builder::new_current_thread()
    //                     .build()
    //                     .unwrap()
    //                     .block_on(value.lock())
    //                     .label()
    //             {
    //                 Some(value.clone())
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect()
    // }
    pub async fn find_all_by_label(&self, label: &str) -> Vec<&T> {
        self.items
            .iter()
            .filter_map(|(_, value)| {
                if label
                    == value.label()
                {
                    Some(value)
                } else {
                    None
                }
            })
            .collect()
    }

    /// return max one element with matching discriminator
    pub fn find_by_discrim(&self, discrim: &Discriminator) -> Option<&T> {
        self.items.get(discrim)
    }

    /// return max one element with matching discriminator (mutable)
    pub fn find_by_discrim_mut(&mut self, discrim: &Discriminator) -> Option<&mut T> {
        self.items.get_mut(discrim)
    }

    /// check if the item is in collection
    pub fn contains(&self, discrim: &Discriminator) -> bool {
        self.items.contains_key(discrim)
    }

    /// insert an item and return handle to it
    pub fn insert(&mut self, item: T) -> Option<T> {
        self.items.insert(item.discrim().clone(), item)
    }

    /// removes an item by discrim
    pub fn remove(&mut self, discrim: &Discriminator) -> bool {
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
