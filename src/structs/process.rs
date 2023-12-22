use std::{
    collections::HashMap,
    error::Error,
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::PathBuf,
    process::Stdio,
    sync::Arc,
};

use async_trait::async_trait;
use tokio::{
    process::{Child, Command},
    sync::{
        mpsc::{self, UnboundedSender},
        oneshot, Mutex,
    },
    task::JoinHandle,
};

use crate::traits::Component;

use super::{
    Discriminator, Event, EventSerde, Pool, Request, RequestContent, Response, ResponseContent,
    Storage, Subscriptions,
};

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

    /// handle to the task responsible for listening to requests
    listener: JoinHandle<()>,

    /// handle to the task responsible for responding
    responder: JoinHandle<()>,

    /// path to response socket
    res: UnboundedSender<Response>,

    /// event confirm recieve senders
    confirm_handles: Arc<Mutex<HashMap<u32, oneshot::Sender<Request>>>>,
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
        let child = Command::new(&command)
            .kill_on_drop(true)
            .args(&args)
            .current_dir(storage.path())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let (responder_send, mut responder_recv): (UnboundedSender<Response>, _) =
            mpsc::unbounded_channel();
        let responder = tokio::spawn(async move {
            let mut socket = None;

            while let Some(mut res) = responder_recv.recv().await {
                if let ResponseContent::SetSocket(path) = res.content() {
                    socket = Some(path.to_owned());
                    continue;
                    // res = Response::new_with_request(
                    //     ResponseContent::Success {
                    //         content: super::ResponseSuccess::ListenerSet,
                    //     },
                    //     res.id(),
                    // )
                }

                if let Some(socket) = &socket {
                    if let Ok(mut stream) = UnixStream::connect(socket) {
                        stream
                            .write_all(serde_json::to_vec(&res).unwrap().as_slice())
                            .unwrap();
                        stream.flush().unwrap();
                    }
                }
            }
        });

        let listener = {
            let responder = responder_send.clone();
            tokio::spawn(async move {
                let socket =
                    tokio::task::block_in_place(|| UnixListener::bind(socket_path).unwrap());
                let mut incoming = socket.incoming();

                while let Some(stream) = tokio::task::block_in_place(|| incoming.next()) {
                    let mut stream = match stream {
                        Ok(stream) => stream,
                        Err(_) => continue,
                    };

                    let mut msg = String::new();
                    if stream.read_to_string(&mut msg).is_err() {
                        continue;
                    }

                    let request: Request = match serde_json::from_str(&msg) {
                        Ok(req) => req,
                        Err(_) => continue,
                    };

                    let res = request.send().await;

                    responder.send(res).unwrap();
                }
            })
        };

        Ok(Self {
            child,
            label,
            storage,
            pool: Pool::default(),
            discrim,
            subscriptions: Subscriptions::default(),
            command: [command].into_iter().chain(args).collect(),
            listener,
            responder,
            res: responder_send,
            confirm_handles: Arc::new(Mutex::new(HashMap::default())),
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

    async fn pass(&mut self, event: &mut Event) -> bool {
        match event {
            Event::RequestPacket(ref mut packet) => {
                match packet.get().content() {
                    RequestContent::SetSocket { path } => {
                        self.res
                            .send(Response::new_with_request(
                                ResponseContent::SetSocket(self.storage.path().join(path)),
                                *packet.get().id(),
                            ))
                            .unwrap();
                        packet.respond(Response::new_with_request(
                            ResponseContent::Success {
                                content: super::ResponseSuccess::ListenerSet,
                            },
                            *packet.get().id(),
                        ));
                    }
                    RequestContent::ConfirmRecieve { id, pass } => unreachable!("idk"),
                    RequestContent::Subscribe { channel, priority } => {
                        if self.subscriptions.contains(channel) {
                            packet.respond(Response::new_with_request(
                                ResponseContent::Error {
                                    content: super::ResponseError::AlreadySubscribed,
                                },
                                *packet.get().id(),
                            ));
                            return false;
                        } else {
                            self.subscriptions.insert(channel.clone());
                            Event::send(Event::RegSubscription(
                                channel.clone(),
                                *priority,
                                self.discrim.clone(),
                            ));
                            packet.respond(Response::new_with_request(
                                ResponseContent::Success {
                                    content: super::ResponseSuccess::SubscribeAdded,
                                },
                                *packet.get().id(),
                            ));
                            return false;
                        }
                    }
                }
                return false;
            }
            _ => {}
        }

        let resp = Response::new(ResponseContent::Event {
            content: EventSerde::from_event(event),
        });
        let (tx, rx) = oneshot::channel();
        self.confirm_handles.lock().await.insert(resp.id(), tx);
        self.res.send(resp).unwrap();
        match rx.await {
            Ok(req) => match req.content() {
                RequestContent::ConfirmRecieve { id: _, pass } => return *pass,
                RequestContent::Subscribe { channel, priority } => {
                    if self.subscriptions.contains(channel) {
                        self.res
                            .send(Response::new_with_request(
                                ResponseContent::Error {
                                    content: super::ResponseError::AlreadySubscribed,
                                },
                                *req.id(),
                            ))
                            .unwrap();
                        return false;
                    } else {
                        self.subscriptions.insert(channel.clone());
                        Event::send(Event::RegSubscription(
                            channel.clone(),
                            *priority,
                            self.discrim.clone(),
                        ));
                        return false;
                    }
                }
                RequestContent::SetSocket { path } => {
                    self.res
                        .send(Response::new(ResponseContent::SetSocket(
                            self.storage.path().join(path),
                        )))
                        .unwrap();
                    return false;
                }
            },
            Err(_) => {}
        }

        true
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        self.responder.abort();
        self.listener.abort();
        let _ = self.child.kill();
    }
}
