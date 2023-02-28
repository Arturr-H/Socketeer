/* Imports */
use std::net::SocketAddr;
use crate::peers::{Peers, PeerMap};

/* Main */
pub struct RequestData<'a> {

    /// Which peer is requesting this endpoint?
    peer: SocketAddr,

    /// All the peers connected to this server
    peers: PeerMap,

    /// The request data which was provided client-side
    data: &'a String,
}

/* Method impls */
impl<'a> RequestData<'a> {
    /// Constructor
    pub fn new(peer: SocketAddr, peers: PeerMap, data: &'a String) -> Self {
        Self { peer, peers, data }
    }
}
