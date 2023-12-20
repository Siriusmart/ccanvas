use tokio::sync::{mpsc, oneshot};

/// a packet of info, expecting response
pub struct Packet<T, R> {
    message: T,
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

    pub async fn send(
        self,
        channel: mpsc::Sender<Self>,
    ) -> Result<(), mpsc::error::SendError<Self>> {
        channel.send(self).await
    }
}
