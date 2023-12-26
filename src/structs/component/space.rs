use std::sync::Arc;
use std::{error::Error, io::Write};

use async_trait::async_trait;
use tokio::sync::Mutex;

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
    subspaces: Arc<Mutex<Collection<Self>>>,

    /// currently in use space, could be self or children
    focus: Arc<Mutex<Focus>>,

    /// process event subscriptions in this space
    passes: Arc<Mutex<Passes>>,

    /// process pool
    processes: Arc<Mutex<Collection<Process>>>,
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
            subspaces: Arc::new(Mutex::new(Collection::default())),
            focus: Arc::new(Mutex::new(Focus::default())),
            passes: Arc::new(Mutex::new(Passes::default())),
            processes: Arc::new(Mutex::new(Collection::default())),
        }
    }

    /// start listening to all events, only the top level,
    /// "master" space should do this
    pub async fn listen(arc: Arc<Self>) {
        let mut listener = Event::start();

        while let Some(mut event) = listener.recv().await {
            // write out the event for debugging purposes
            if let Event::RequestPacket(req) = &mut event {
                // req.respond(Response::new_with_request(ResponseContent::Undelivered, *req.get().id())).unwrap();
                // continue;
                if req.get().content()
                    == &(RequestContent::Drop {
                        discrim: Some(Discriminator(vec![1])),
                    })
                    && req.get().target().0.is_empty()
                {
                    return;
                }
            }

            let arc = arc.clone();
            // pass the event to master space
            tokio::spawn(async move {
                arc.pass(&mut event).await;
            });
            // let _ = arc.pass(&mut event).await;
        }
    }

    pub async fn spawn(
        &self,
        label: String,
        command: String,
        args: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        self.processes
            .lock()
            .await
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

    async fn pass(&self, event: &mut Event) -> bool {
        match event {
            // i have no idea what this branch does
            // but i think it is unreachable, but shouldnt cause a panic
            // as a bad request could reach this, so for now just ignore it
            Event::RequestPacket(req) if req.get().target() == self.discrim() => {
                match req.get().content() {
                    RequestContent::Subscribe {
                        channel,
                        priority,
                        component: Some(discrim),
                    } => {
                        if let Some(child) = self.discrim.immediate_child(discrim.clone()) {
                            if self.processes.lock().await.contains(&child) {
                                // if its a process, subscribe to the event right here
                                self.passes.lock().await.subscribe(
                                    channel.clone(),
                                    PassItem::new(discrim.clone(), *priority),
                                );
                                req.respond(Response::new_with_request(
                                    ResponseContent::Success {
                                        content: ResponseSuccess::SubscribeAdded,
                                    },
                                    *req.get().id(),
                                ))
                                .unwrap();
                            }
                        }
                    }
                    RequestContent::Drop { discrim } => {
                        // drop (remove) a child component
                        if let Some(child) = self.discrim.immediate_child(discrim.clone().unwrap())
                        {
                            if self.processes.lock().await.contains(&child) {
                                self.passes.lock().await.unsub_all(&child);
                                self.processes.lock().await.remove(&child);
                            } else if self.subspaces.lock().await.contains(&child) {
                                self.subspaces.lock().await.remove(&child);
                                if *self.focus.lock().await == Focus::Children(child) {
                                    *self.focus.lock().await = Focus::This
                                }
                            } else {
                                req.respond(Response::new_with_request(
                                    ResponseContent::Error {
                                        content: ResponseError::ComponentNotFound,
                                    },
                                    *req.get().id(),
                                ))
                                .unwrap();
                                return false;
                            }
                            req.respond(Response::new_with_request(
                                ResponseContent::Success {
                                    content: ResponseSuccess::Dropped,
                                },
                                *req.get().id(),
                            ))
                            .unwrap();
                        }
                    }
                    RequestContent::Render {
                        content: RenderRequest::SetChar { x, y, c },
                    } => {
                        write!(
                            unsafe { SCREEN.get_mut() }.unwrap(),
                            "{}{c}",
                            termion::cursor::Goto(*x as u16 + 1, *y as u16 + 1)
                        )
                        .unwrap();
                        unsafe { SCREEN.get_mut() }.unwrap().flush().unwrap();
                        req.respond(Response::new_with_request(
                            ResponseContent::Success {
                                content: ResponseSuccess::Rendered,
                            },
                            *req.get().id(),
                        ))
                        .unwrap();
                    }
                    RequestContent::Subscribe { .. }
                    | RequestContent::ConfirmRecieve { .. }
                    | RequestContent::SetSocket { .. } => unreachable!("not requests to spaces"),
                }

                return false;
            } // do stuff
            Event::RequestPacket(req) => {
                // pass the event to "next immediate child"
                // aka the next item it should pass to in order to get the request
                // to its intended target
                if let Some(child) = self.discrim.immediate_child(req.get().target().clone()) {
                    // no 2 components are the same, so order shouldnt matter
                    if let Some(proc) = self.processes.lock().await.find_by_discrim(&child) {
                        proc.pass(event).await;
                    } else if let Some(space) = self.subspaces.lock().await.find_by_discrim(&child)
                    {
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
            _ => {}
        }

        // all components listening to this event
        let targets = self.passes.lock().await.subscribers(event.subscriptions());

        // repeat until someone decide to capture the event
        for target in targets {
            if !self
                .processes
                .lock()
                .await
                .find_by_discrim(target.discrim())
                .unwrap()
                .pass(event)
                .await
            {
                return false;
            }
        }

        // if all went well then continue to pass down into subspaces
        if let Focus::Children(discrim) = &*self.focus.lock().await {
            self.processes
                .lock()
                .await
                .find_by_discrim(discrim)
                .unwrap()
                .pass(event)
                .await;
        }

        true
    }
}
