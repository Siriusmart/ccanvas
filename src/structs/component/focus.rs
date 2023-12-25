use super::Discriminator;

/// if the space is focusing on itself or a child subspace
#[derive(PartialEq, Eq)]
pub enum Focus {
    /// render self, dont pass events further down
    This,
    /// render child, pass events to it
    Children(Discriminator),
}

impl Default for Focus {
    fn default() -> Self {
        Self::This
    }
}
