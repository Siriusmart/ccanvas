use std::{
    error::Error,
    path::{Path, PathBuf},
};

use tokio::fs;

use crate::structs::Discriminator;
use crate::values::ROOT;

/// wrapper struct for storage of a single component
pub struct Storage {
    // absolute path to directory
    path: PathBuf,
}

impl Storage {
    /// creates a new struct and the corresponding directory
    pub async fn new(discrim: &Discriminator) -> Self {
        let path = ROOT.get().unwrap().join(PathBuf::from_iter(
            discrim.as_vec().iter().map(u32::to_string),
        ));

        if !fs::try_exists(&path).await.unwrap() {
            fs::create_dir_all(&path).await.unwrap();
        }

        Self { path }
    }

    /// returns absolute path to storage
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// remove a file or directory at path if it exists
    pub async fn remove_if_exist(path: &Path) -> Result<(), Box<dyn Error>> {
        if fs::try_exists(path).await? {
            if fs::metadata(path).await?.is_dir() {
                fs::remove_dir_all(&path).await?;
            } else {
                fs::remove_file(&path).await?;
            }
        }

        Ok(())
    }
}

impl Drop for Storage {
    fn drop(&mut self) {
        let path = self.path.clone();
        let _ = std::fs::remove_dir_all(path);
    }
}
