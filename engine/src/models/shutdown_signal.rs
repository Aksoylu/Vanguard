use tokio::sync::watch::channel as TokioChannel;
use tokio::sync::watch::Receiver as TokioReceiver;
use tokio::sync::watch::Sender as TokioSender;

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
