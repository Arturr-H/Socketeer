/* Imports */
use futures_channel::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use std::{
    sync::{ Arc, Mutex, LockResult, MutexGuard },
    collections::HashMap,
    net::SocketAddr
};

/* Type aliases */
type Tx = UnboundedSender<Message>;
pub type PeerMap = HashMap<SocketAddr, Tx>;

/// Each connection is called a `peer`. Each peer has 
/// a `std::net::SocketAddr` tied to it and an `futures_channel::mpsc::UnboundedSender`
/// which allows for sending messages to that peer.
pub struct Peers(Arc<Mutex<PeerMap>>);

/* Method impls */
impl Peers {
    /// Try locking the peer map and getting inner values (mutex)
    pub fn lock(&self) -> Result<PeerMap, ()> {
        match self.0.lock() {
            Ok(e) => Ok(*e),
            Err(_) => Err(())
        }
    }
}
