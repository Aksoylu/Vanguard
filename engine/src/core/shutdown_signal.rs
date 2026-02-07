use tokio::sync::watch::Sender as TokioSender;
use tokio::sync::watch::Receiver as TokioReceiver;
use tokio::sync::watch::channel as TokioChannel;

#[derive(Clone)]
pub struct ShutdownSignal {
    pub publisher: TokioSender<bool>,
    pub subscriber: TokioReceiver<bool>,
}

impl ShutdownSignal {
    pub fn new() -> Self {
        let (publisher, subscriber) = TokioChannel(false);
        Self {
            publisher,
            subscriber,
        }
    }
}