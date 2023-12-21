use std::{error::Error, os::unix::net::UnixListener, process::Stdio};

use async_trait::async_trait;
use tokio::process::{Child, Command};

use crate::traits::Component;

use super::{Event, Pool, Storage, Subscriptions, Discriminator};

/// single runnable process
pub struct Process {
    /// name of the current process
    label: String,

    /// unique identifier of the current process
    discrim: Discriminator,

    /// data storage for self
    pool: Pool,

    /// shared storage folder for self
    storage: Storage,

    /// subscribed events to be passed to process
    subscriptions: Subscriptions,

    /// command that was ran
    command: Vec<String>,

    /// process handle
    child: Child,
}

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.discrim == other.discrim
    }
}

impl Process {
    /// spawns a new process with command
    pub async fn spawn(
        label: String,
        parent: &Discriminator,
        command: String,
        args: Vec<String>,
    ) -> Result<Self, Box<dyn Error>> {
        let discrim = parent.new_child();
        let storage = Storage::new(&discrim).await;

        let socket_path = storage.path().join("requests.sock");
        Storage::remove_if_exist(&socket_path).await.unwrap();

        let socket = UnixListener::bind(socket_path)?;

        Ok(Self {
            child: Command::new(&command)
                .kill_on_drop(true)
                .args(&args)
                .current_dir(storage.path())
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?,
            label,
            storage,
            pool: Pool::default(),
            discrim ,
            subscriptions: Subscriptions::default(),
            command: [command].into_iter().chain(args).collect(),
        })
    }
}

#[async_trait]
impl Component for Process {
    fn label(&self) -> &str {
        &self.label
    }

    fn discrim(&self) -> &Discriminator {
        &self.discrim
    }

    fn pool(&self) -> &Pool {
        &self.pool
    }

    fn storage(&self) -> &Storage {
        &self.storage
    }

    async fn pass(&mut self, event: &Event) -> bool {
        todo!()
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        let _ = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.child.kill());
    }
}
