/* Imports */
use std::net::SocketAddr;
use serde::Deserialize;
use crate::peers::{ Peers, PeerMap };

/* Main */
pub struct Request<'a> {

    /// Which peer is requesting this endpoint?
    peer: SocketAddr,

    /// All the peers connected to this server
    peers: &'a PeerMap,

    /// The request data which was provided client-side
    data: &'a String,
}

/* Method impls */
impl<'a> Request<'a> {
    /// Constructor
    pub fn new(peer: SocketAddr, peers: &'a PeerMap, data: &'a String) -> Self {
        Self { peer, peers, data }
    }

    /* Getters */
    /// Which peer is requesting this endpoint?
    pub fn peer(&self) -> SocketAddr { self.peer }

    /// All the peers connected to this server
    pub fn peers(&self) -> &PeerMap { &&self.peers }

    /// The request data which was provided client-side
    /// 
    /// `T`: Is the struct that we want to recieve from client side.
    /// That struct has to implement `serde::Deserialize`.
    /// 
    /// ## Examples
    /// use serde::Deserialize;
    /// 
    /// ```
    /// #[derive(Deserialize)]
    /// struct ClientData {
    ///     message: String,
    ///     id: usize
    /// }
    /// 
    /// /* Endpoint function */
    /// fn some_endpoint(req: Request) -> Response {
    ///     let data = req.data::<ClientData>().unwrap();
    ///     
    ///     Response::json("Recieved data!").to_origin(req.peer())
    /// }
    /// ```
    pub fn data<T: Deserialize<'a>>(&self) -> Option<T> { serde_json::from_str::<T>(&self.data).ok() }
}
