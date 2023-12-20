use std::{
    io::Write,
    path::PathBuf,
};

use async_trait::async_trait;

use crate::{
    traits::Component,
    values::{discrim, SCREEN},
};

use super::{Collection, Event, Focus, KeyCode, KeyEvent, KeyModifier, Pool, Storage};

/// the basic unit of display
pub struct Space {
    /// name of the current space
    label: String,

    /// unique identifier of the current space - a "path" of u32s
    discrim: Vec<u32>,

    /// data storage for children
    pool: Pool,

    /// shared storage folder for space children
    storage: Storage,

    /// spaces the current space contains
    subspaces: Collection<Self>,

    /// currently in use space, could be self or children
    focus: Focus,
}

impl Space {
    pub fn new(label: String) -> Self {
        Self {
            storage: Storage::new(PathBuf::from(&label)),
            label,
            discrim: vec![discrim()],
            pool: Pool::default(),
            subspaces: Collection::<Self>::default(),
            focus: Focus::default(),
        }
    }

    /// create new self with parent discriminator
    fn new_with_parent(label: String, parent_discrim: &Vec<u32>) -> Self {
        let mut parent_discrim = parent_discrim.clone();
        parent_discrim.push(discrim());
        Self {
            storage: Storage::new(PathBuf::from(&label)),
            label,
            discrim: parent_discrim,
            pool: Pool::default(),
            subspaces: Collection::<Self>::default(),
            focus: Focus::default(),
        }
    }

    /// start listening to all events, only the top level,
    /// "master" space should do this
    pub async fn listen(&mut self) {
        let mut listener = Event::listen();

        while let Ok(event) = listener.recv().await {
            write!(
                unsafe { SCREEN.get_mut().unwrap() },
                "{}{}{:?}",
                termion::cursor::Goto(1, 1),
                termion::clear::CurrentLine,
                event
            )
            .unwrap();
            unsafe { SCREEN.get_mut() }.unwrap().flush().unwrap();
            if event == Event::KeyPress(KeyEvent::new(KeyCode::Char('q'), KeyModifier::None)) {
                return;
            }
        }
    }
}

#[async_trait]
impl Component for Space {
    fn label(&self) -> &str {
        &self.label
    }

    fn discrim(&self) -> &Vec<u32> {
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
