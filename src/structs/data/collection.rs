use std::collections::HashMap;
use std::sync::Arc;

use crate::structs::Discriminator;
use crate::traits::Component;

/// a collection of component items
pub struct Collection<T: Component> {
    // items: HashMap<Discriminator, Arc<Mutex<T>>>,
    items: HashMap<Discriminator, Arc<T>>,
}

impl<T: Component> Collection<T> {
    /// return all elements with that label
    pub async fn find_all_by_label(&self, label: &str) -> Vec<&T> {
        self.items
            .iter()
            .filter_map(|(_, value)| {
                if label == value.label() {
                    Some(value.as_ref())
                } else {
                    None
                }
            })
            .collect()
    }

    /// return max one element with matching discriminator
    pub fn find_by_discrim(&self, discrim: &Discriminator) -> Option<&T> {
        self.items.get(discrim).map(|item| item.as_ref())
    }

    /// return max one element with matching discriminator (mutable)
    // pub fn find_by_discrim_mut(&mut self, discrim: &Discriminator) -> Option<&mut T> {
    //     self.items.get_mut(discrim)
    // }

    /// check if the item is in collection
    pub fn contains(&self, discrim: &Discriminator) -> bool {
        self.items.contains_key(discrim)
    }

    /// insert an item and return handle to it
    pub fn insert(&mut self, item: T) {
        self.items.insert(item.discrim().clone(), Arc::new(item));
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
