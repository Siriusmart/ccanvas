use serde::{Serialize, Deserialize};
use tokio::sync::OnceCell;

static mut DISCRIM: OnceCell<u32> = OnceCell::const_new_with(0);
pub fn discrim() -> u32 {
    let discrim = unsafe { DISCRIM.get_mut().unwrap() };
    *discrim += 1;
    *discrim
}

/// a unique path id for every component
#[derive(Default, PartialEq, Eq, Clone, Debug, Serialize, Deserialize, Hash)]
pub struct Discriminator(Vec<u32>);

impl Discriminator {
    /// create new child component
    pub fn new_child(&self) -> Self {
        let mut new_discrim = self.0.to_vec();
        new_discrim.push(discrim());
        Self(new_discrim)
    }

    /// returns internal vec
    pub fn as_vec(&self) -> &Vec<u32> {
        &self.0
    }

    // pub fn truncate(mut self, len: usize) -> Self {
    //     self.0.truncate(len);
    //     self
    // }
}
