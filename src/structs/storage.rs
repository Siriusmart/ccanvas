use std::path::PathBuf;

/// wrapper struct for storage of a single component
pub struct Storage {
    path: PathBuf,
}

impl Storage {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}
