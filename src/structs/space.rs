use std::io::Write;

use async_trait::async_trait;

use crate::{
    traits::Component,
    values::SCREEN,
};

use super::{Collection, Event, Focus, KeyCode, KeyEvent, KeyModifier, Passes, Pool, Storage, Discriminator, Process};

/// the basic unit of display
pub struct Space {
    /// name of the current space
    label: String,

    /// unique identifier of the current space - a "path" of u32s
    discrim: Discriminator,

    /// data storage for children
    pool: Pool,

    /// shared storage folder for space children
    storage: Storage,

    /// spaces the current space contains
    subspaces: Collection<Self>,

    /// currently in use space, could be self or children
    focus: Focus,

    /// process event subscriptions in this space
    passes: Passes,

    /// process pool
    processes: Collection<Process>,
}

impl Space {
    pub async fn new(label: String) -> Self {
        Self::new_with_parent(label, &Discriminator::default()).await
    }

    /// create new self with parent discriminator
    async fn new_with_parent(label: String, parent_discrim: &Discriminator) -> Self {
        Self {
            storage: Storage::new(&parent_discrim).await,
            label,
            discrim: parent_discrim.new_child(),
            pool: Pool::default(),
            subspaces: Collection::default(),
            focus: Focus::default(),
            passes: Passes::default(),
            processes: Collection::default()
        }
    }

    /// start listening to all events, only the top level,
    /// "master" space should do this
    pub async fn listen(&mut self) {
        let mut listener = Event::start();

        while let Some(event) = listener.recv().await {
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

            let _ = self.pass(&event).await;
        }
    }
}

#[async_trait]
impl Component for Space {
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
        match event {
            Event::RequestMessage(req) if req.target() == self.discrim() => {} // do stuff
            Event::RequestPacket(req) if req.get().target() == self.discrim() => {} // do stuff
            _ => {}
        }

        let targets = self.passes.subscribers(event.subscriptions());

        for target in targets {
            if !self.processes.find_by_discrim_mut(target.discrim()).unwrap().pass(event).await {
                return false;
            }
        }

        if let Focus::Children(discrim) = &self.focus {
            let _ = self.processes.find_by_discrim_mut(discrim).unwrap().pass(event);
        }

        true
    }
}
