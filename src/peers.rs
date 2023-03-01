/* Imports */
use futures_channel::mpsc::UnboundedSender;
use tokio::sync::{Mutex, MutexGuard};
use tokio_tungstenite::tungstenite::Message;
use std::{
    sync::Arc,
    collections::HashMap,
    net::SocketAddr
};

/* Type aliases */
type Tx = UnboundedSender<Message>;
pub type PeerMap = HashMap<SocketAddr, Tx>;

/// Each connection is called a `peer`. Each peer has 
/// a `std::net::SocketAddr` tied to it and an `futures_channel::mpsc::UnboundedSender`
/// which allows for sending messages to that peer.
#[derive(Clone)]
pub struct Peers(Arc<Mutex<PeerMap>>);

/* Method impls */
impl Peers {
    /// Construct a new peers map
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }

    /// Try locking the peer map and getting inner values (mutex)
    pub async fn lock(&self) -> MutexGuard<PeerMap> {
        self.0.lock().await
    }
}
