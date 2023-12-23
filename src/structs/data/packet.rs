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

    /// respond to packet sender, and return Ok if sent successfully
    pub fn respond(&mut self, res: R) -> Result<(), crate::Error> {
        if let Some(resp) = std::mem::take(&mut self.responder) {
            // this allows &mut Self to use
            // this function
            let _ = resp.send(res);
            Ok(())
        } else {
            Err(crate::Error::PacketDoubleResp)
        }
    }
}

impl<T, R> PartialEq for Packet<T, R> {
    fn eq(&self, _other: &Self) -> bool {
        false // no 2 packets are the same
    }
}
