use tokio::sync::oneshot;

/// a packet of info, expecting response
#[derive(Debug)]
pub struct Packet<T, R> {
    /// actual packet data
    message: T,
    /// a "callback" sender
    responder: Option<oneshot::Sender<R>>,
}

impl<T, R> Packet<T, R> {
    /// create a packet and a receiver handle
    pub fn new(message: T) -> (Self, oneshot::Receiver<R>) {
        let (tx, rx) = oneshot::channel();

        (
            Self {
                message,
                responder: Some(tx),
            },
            rx,
        )
    }

    /// returns inner conent
    pub fn get(&self) -> &T {
        &self.message
    }

    pub fn respond(&mut self, res: R) -> bool {
        if let Some(resp) = std::mem::take(&mut self.responder) {
            let _ = resp.send(res);
            return true;
        } else {
            return false;
        }
    }
}

impl<T, R> PartialEq for Packet<T, R> {
    fn eq(&self, _other: &Self) -> bool {
        false // no 2 packets are the same
    }
}
