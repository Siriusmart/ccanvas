use std::{error::Error, io::Write};

use async_trait::async_trait;

use crate::{traits::Component, values::SCREEN};

use crate::structs::*;

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
            storage: Storage::new(parent_discrim).await,
            label,
            discrim: parent_discrim.new_child(),
            pool: Pool::default(),
            subspaces: Collection::default(),
            focus: Focus::default(),
            passes: Passes::default(),
            processes: Collection::default(),
        }
    }

    /// start listening to all events, only the top level,
    /// "master" space should do this
    pub async fn listen(&mut self) {
        let mut listener = Event::start();

        while let Some(mut event) = listener.recv().await {
            // write out the event for debugging purposes
            write!(
                unsafe { SCREEN.get_mut().unwrap() },
                "{}{}{:?}",
                termion::cursor::Goto(1, 1),
                termion::clear::CurrentLine,
                event
            )
            .unwrap();
            unsafe { SCREEN.get_mut() }.unwrap().flush().unwrap();

            // q is the exit key for debugging purposes
            if event == Event::KeyPress(KeyEvent::new(KeyCode::Char('q'), KeyModifier::None)) {
                return;
            }

            // pass the event to master space
            let _ = self.pass(&mut event).await;
        }
    }

    pub async fn spawn(
        &mut self,
        label: String,
        command: String,
        args: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        self.processes
            .insert(Process::spawn(label, &self.discrim, command, args).await?);
        Ok(())
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

    async fn pass(&mut self, event: &mut Event) -> bool {
        match event {
            // i have no idea what this branch does
            // but i think it is unreachable, but shouldnt cause a panic
            // as a bad request could reach this, so for now just ignore it
            Event::RequestPacket(req) if req.get().target() == self.discrim() => return true, // do stuff
            Event::RequestPacket(req) => {
                // pass the event to "next immediate child"
                // aka the next item it should pass to in order to get the request
                // to its intended target
                if let Some(child) = self.discrim.immediate_child(req.get().target().clone()) {
                    // no 2 components are the same, so order shouldnt matter
                    if let Some(proc) = self.processes.find_by_discrim_mut(&child) {
                        proc.pass(event).await;
                    } else if let Some(space) = self.subspaces.find_by_discrim_mut(&child) {
                        space.pass(event).await;
                    } else {
                        req.respond(Response::new(ResponseContent::Undelivered))
                            .unwrap();
                    }

                    return false;
                }
                // otherwise self is not a parent to the target component
                // and something went wrong
            }
            Event::RegSubscription(sub, priority, discrim) => {
                if let Some(child) = self.discrim.immediate_child(discrim.clone()) {
                    if self.processes.contains(&child) {
                        // if its a process, subscribe to the event right here
                        self.passes
                            .subscribe(sub.clone(), PassItem::new(discrim.clone(), *priority));
                    } else if let Some(space) = self.subspaces.find_by_discrim_mut(&child) {
                        // if its a process owned by a subspace
                        // let the subspace handle it
                        space.pass(event).await;
                    }
                } else {
                    panic!("bad subscription, not delivered")
                }
                return false;
            }
            _ => {}
        }

        // all components listening to this event
        let targets = self.passes.subscribers(event.subscriptions());

        // repeat until someone decide to capture the event
        for target in targets {
            if !self
                .processes
                .find_by_discrim_mut(target.discrim())
                .unwrap()
                .pass(event)
                .await
            {
                return false;
            }
        }

        // if all went well then continue to pass down into subspaces
        if let Focus::Children(discrim) = &self.focus {
            self.processes
                .find_by_discrim_mut(discrim)
                .unwrap()
                .pass(event)
                .await;
        }

        true
    }
}
