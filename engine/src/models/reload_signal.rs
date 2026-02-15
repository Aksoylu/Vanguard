use tokio::sync::watch::channel as TokioChannel;
use tokio::sync::watch::Receiver as TokioReceiver;
use tokio::sync::watch::Sender as TokioSender;

#[derive(Clone)]
pub struct ReloadSignal {
    pub publisher: TokioSender<bool>,
    pub subscriber: TokioReceiver<bool>,
}

impl ReloadSignal {
    pub fn new() -> Self {
        let (publisher, subscriber) = TokioChannel(false);
        Self {
            publisher,
            subscriber,
        }
    }

    pub fn trigger(&self) {
        let _ = self.publisher.send(true);
        // Reset to false so future waits can trigger again
        let _ = self.publisher.send(false);
    }
}
