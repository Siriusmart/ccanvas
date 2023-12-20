use std::sync::{Arc, Mutex};

use super::Space;

pub enum Focus {
    This,
    Children(Arc<Mutex<Space>>),
}

impl Default for Focus {
    fn default() -> Self {
        Self::This
    }
}
