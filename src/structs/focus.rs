use std::sync::{Arc, Mutex};

use super::Space;

/// if the space is focusing on itself or a child subspace
pub enum Focus {
    /// render self, dont pass events further down
    This,
    /// render child, pass events to it
    Children(Arc<Mutex<Space>>),
}

impl Default for Focus {
    fn default() -> Self {
        Self::This
    }
}
