use tokio::sync::oneshot;

/// a packet of info, expecting response
#[derive(Debug)]
pub struct Packet<T, R> {
    /// actual packet data
    message: T,
    /// a "callback" sender
    responder: oneshot::Sender<R>,
}

impl<T, R> Packet<T, R> {
    /// create a packet and a receiver handle
    pub fn new(message: T) -> (Self, oneshot::Receiver<R>) {
        let (tx, rx) = oneshot::channel();

        (
            Self {
                message,
                responder: tx,
            },
            rx,
        )
    }

    /// returns inner conent
    pub fn get(&self) -> &T {
        &self.message
    }
}

impl<T, R> PartialEq for Packet<T, R> {
    fn eq(&self, _other: &Self) -> bool {
        false // no 2 packets are the same
    }
}
