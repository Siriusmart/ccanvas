use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
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

use crate::structs::*;

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

    /// command that was ran
    command: Vec<String>,

    /// process handle
    child: Arc<Mutex<Child>>,

    /// handle to the task responsible for listening to requests
    listener: JoinHandle<()>,

    /// handle to the task responsible for responding
    responder: JoinHandle<()>,

    /// path to response socket
    res: UnboundedSender<Response>,

    /// event confirm recieve senders
    confirm_handles: Arc<Mutex<HashMap<u32, oneshot::Sender<bool>>>>,
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

        // a new sender is pushed to the map whenever something is sent to the component
        // the sender returns a boolean, if true then the event will not be captured, vice versa
        // the important part is that this will hold the process until a confirmation message is
        // recieved
        let confirm_handles: Arc<Mutex<HashMap<u32, oneshot::Sender<bool>>>> =
            Arc::new(Mutex::new(HashMap::default()));

        // the component should send requests to this path
        let socket_path = storage.path().join("requests.sock");
        Storage::remove_if_exist(&socket_path).await.unwrap();
        let child = Arc::new(Mutex::new(
            Command::new(&command)
                .kill_on_drop(true)
                .args(&args)
                .current_dir(storage.path())
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?,
        ));

        let (responder_send, mut responder_recv): (UnboundedSender<Response>, _) =
            mpsc::unbounded_channel();

        // the responder task recieve Response
        // serialise it and send it to the component, if it specified a socket to send to
        let responder = {
            let confirm_handles = confirm_handles.clone();
            let child = child.clone();
            let discrim = discrim.clone();
            tokio::spawn(async move {
                // by default there is no socket
                let mut socket = None;
                // let child = child;

                while let Some(res) = responder_recv.recv().await {
                    let confirm_handles = confirm_handles.clone();
                    // this special "response" is recieved when a SetSocket request
                    // is sent by the component
                    if let ResponseContent::SetSocket(path) = res.content() {
                        socket = Some(path.to_owned());
                        continue;
                    }

                    // only send a message when a socket is specified
                    if let Some(socket) = &socket {
                        let socket = socket.clone();
                        let child = child.clone();
                        let discrim = discrim.clone();
                        tokio::spawn(async move {
                            // check if the child process has crashed
                            if child.lock().await.try_wait().unwrap().is_some() {
                                Event::send(Event::RequestPacket(
                                    Packet::new(Request::new(
                                        discrim.clone().immediate_parent().unwrap(),
                                        RequestContent::Drop {
                                            // if true, then tell the parent space to drop it
                                            discrim: Some(discrim),
                                        },
                                    ))
                                    .0,
                                ))
                            }

                            if let Ok(mut stream) = UnixStream::connect(socket) {
                                std::fs::OpenOptions::new()
                                    .create(true)
                                    .append(true)
                                    .open("log.txt")
                                    .unwrap()
                                    .write_all(format!("sent {res:?}\n").as_bytes())
                                    .unwrap();
                                stream
                                    .write_all(serde_json::to_vec(&res).unwrap().as_slice())
                                    .unwrap();
                                stream.flush().unwrap();
                            } else {
                                // if send failed, it is impossible to get a response message
                                confirm_handles.lock().await.remove(&res.id());
                            }
                        });
                    }
                }
            })
        };

        // the listener listens to event and handles it, that all
        let listener = {
            let discrim = discrim.clone();
            let confirm_handles = confirm_handles.clone();
            let responder = responder_send.clone();
            tokio::spawn(async move {
                // creates a socket and listens to it
                let socket =
                    tokio::task::block_in_place(|| UnixListener::bind(socket_path).unwrap());
                let mut incoming = socket.incoming();

                while let Some(stream) = tokio::task::block_in_place(|| incoming.next()) {
                    // give up if the stream is errorneous
                    let mut stream = match stream {
                        Ok(stream) => stream,
                        Err(_) => continue,
                    };

                    let mut msg = String::new();
                    // another chance to give up
                    if stream.read_to_string(&mut msg).is_err() {
                        continue;
                    }

                    // a third chance to give up
                    let mut request: Request = match serde_json::from_str(&msg) {
                        Ok(req) => req,
                        Err(_) => continue,
                    };

                    std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("log.txt")
                        .unwrap()
                        .write_all(format!("recieved {request:?}\n").as_bytes())
                        .unwrap();
                    // if the request is a confirmation to a response
                    // then confirm the response and unblock the self.pass() thing
                    // by sending a message

                    if let RequestContent::ConfirmRecieve { id, pass } = request.content() {
                        let confirm_handles = confirm_handles.clone();
                        let id = *id;
                        let pass = *pass;
                        tokio::spawn(async move {
                            if let Entry::Occupied(entry) = confirm_handles.lock().await.entry(id) {
                                let _ = entry.remove_entry().1.send(pass);
                            }
                        });
                        continue;
                    }

                    match request.content() {
                        RequestContent::ConfirmRecieve { .. } => unreachable!(),
                        RequestContent::Subscribe { .. } | RequestContent::SetSocket { .. } => {
                            *request.target_mut() = discrim.clone()
                        }
                        RequestContent::Drop { discrim: to_drop } => {
                            let to_drop = to_drop.as_ref().unwrap_or(&discrim).clone();
                            *request.target_mut() = to_drop.clone().immediate_parent().unwrap();
                            *request.content_mut() = RequestContent::Drop {
                                discrim: Some(to_drop),
                            };
                        }
                        RequestContent::Render {
                            content: RenderRequest::SetChar { .. },
                        } => {
                            *request.target_mut() = Discriminator::master();
                        }
                    }

                    let responder = responder.clone();
                    tokio::task::spawn(async move {
                        // otherwise, the request gets sended to the master space
                        // and starts propagating downwards
                        let res = request.send().await;

                        // send a response to the request
                        // but requires no confirmation
                        // because the response is already a sort of confirmation
                        responder.send(res).unwrap();
                    });
                }
            })
        };

        Ok(Self {
            child,
            label,
            storage,
            pool: Pool::default(),
            discrim,
            // subscriptions: Subscriptions::default(),
            command: [command].into_iter().chain(args).collect(),
            listener,
            responder,
            res: responder_send,
            confirm_handles,
        })
    }

    pub async fn handle(&self, packet: &mut Packet<Request, Response>) {
        match packet.get().content() {
            // if it is a setsocket
            RequestContent::SetSocket { path } => {
                // tell the responder to use that socket
                self.res
                    .send(Response::new_with_request(
                        ResponseContent::SetSocket(self.storage.path().join(path)),
                        *packet.get().id(),
                    ))
                    .unwrap();
                // and resolve the packet
                packet
                    .respond(Response::new_with_request(
                        ResponseContent::Success {
                            // this can never fail
                            content: ResponseSuccess::ListenerSet,
                        },
                        *packet.get().id(), // this is a response
                    ))
                    .unwrap();
            }
            RequestContent::Subscribe {
                channel,
                priority,
                component: _,
            } => {
                // first add the channel to self as a record
                // and send a register event to the master space
                // which is eventually get sent to the parent space
                // and get added as into the passes
                let mut request = packet.get().clone();
                *request.target_mut() = self.discrim.clone().immediate_parent().unwrap();
                *request.content_mut() = RequestContent::Subscribe {
                    channel: channel.clone(),
                    priority: *priority,
                    component: Some(self.discrim.clone()),
                };
                let (event, recv): (Packet<Request, Response>, _) = Packet::new(request);
                Event::send(Event::RequestPacket(event));
                packet
                    .respond(Response::new_with_request(
                        ResponseContent::Success {
                            content: ResponseSuccess::SubscribeAdded,
                        },
                        *packet.get().id(),
                    ))
                    .unwrap();

                std::mem::drop(tokio::spawn(async { recv.await.unwrap() }));
                // this must not block
                // as it also wait for the
                // current functions to
                // finish
            }
            // confirmreceive gets filtered out and handles in the listener loop
            // so we will never get it
            RequestContent::ConfirmRecieve { .. }
            | RequestContent::Drop { .. }
            | RequestContent::Render { .. } => {
                unreachable!("not a real request")
            }
        }
    }

    /// send a response and wait for confirmation
    pub async fn send_event(&self, resp: Response) -> Result<bool, crate::Error> {
        let (tx, rx) = oneshot::channel();
        self.confirm_handles.lock().await.insert(resp.id(), tx);
        self.res.send(resp).unwrap();
        rx.await.map_err(|_| crate::Error::Undelivered)
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

    async fn pass(&self, event: &mut Event) -> bool {
        // requestpacket is a request, not an event in a real sense
        // and it doesnt serialise into EventSerde either
        // so best just handle it out and filter it first
        if let Event::RequestPacket(packet) = event {
            self.handle(packet).await;
            return false;
        }

        let resp = Response::new(ResponseContent::Event {
            content: EventSerde::from_event(event),
        });

        // send and blocks
        self.send_event(resp).await.unwrap_or(true)
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        self.responder.abort();
        self.listener.abort();
    }
}
