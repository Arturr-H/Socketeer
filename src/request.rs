use core::task;
/* Imports */
use std::{net::SocketAddr, sync::Arc, any::Any, error::Error};
use serde::Deserialize;
use tokio::{runtime::Handle, task::block_in_place};
use crate::peers::{ Peers, PeerMap };

/* Main */
pub struct Request<'a> {

    /// Which peer is requesting this endpoint?
    peer: SocketAddr,

    /// All the peers connected to this server
    peers: &'a PeerMap,

    /// The request data which was provided client-side
    data: &'a String,

    /// The global data which is stored between all endpoints
    global: Arc<tokio::sync::Mutex<(dyn Any + Send)>>
}

/* Method impls */
impl<'a> Request<'a> {
    /// Constructor
    pub(crate) fn new(peer: SocketAddr, peers: &'a PeerMap, data: &'a String, global: Arc<tokio::sync::Mutex<(dyn Any + Send)>>) -> Self {
        Self { peer, peers, data, global }
    }

    /* Getters */
    /// Which peer is requesting this endpoint?
    pub fn peer(&self) -> SocketAddr { self.peer }

    /// All the peers connected to this server
    pub fn peers(&self) -> &PeerMap { &&self.peers }

    /// This function takes a closure as input, with one parameter - `f`.
    /// `f` has one parameter, which is the mutable reference to the global data.
    /// Therefore you can freely change the data as you wish inside of the 
    /// closure body.
    /// 
    /// ## Examples
    /// ```
    /// #[tokio::main]
    /// async fn main() -> () {
    ///     server::Server::new()
    ///         .endpoint("some_endpoint", some_endpoint)
    /// 
    ///         /* Set the global data shared between all endpoints */
    ///         .set_global::<Vec<u8>>(Vec::new())
    ///         .start().await;
    /// }
    /// fn endpoint(req: Request) -> Response {
    ///     req.global_mut::<Vec<u8>>(|v| {
    ///         v.push(1);
    ///     });
    ///     
    ///     Response::json("Changed the global data!").to_all()
    /// }
    /// ```
    /// 
    /// ## Panics
    /// This method will panic if the generic type `T` isn't equal to the global data type
    pub fn global_mut<T: 'static>(&self, f: fn(&mut T) -> ()) -> () {
        block_in_place(move || {
            Handle::current().block_on(async move {
                f(
                    self.global
                        .lock()
                        .await
                        .downcast_mut()
                        .expect("That global variable doesn't seem to exist!")
                )
            })
        })
    }

    /// Get the global data
    /// 
    /// ## Examples
    /// ```
    /// req.global::<Vec<u8>>();
    /// ```
    /// 
    /// ## Panics
    /// This method will panic if the generic type `T` isn't equal to the global data type
    pub fn global<T: 'static + Clone>(&'a self) -> T {
        block_in_place(move || {
            Handle::current().block_on(async move {
                self.global
                    .as_ref()
                    .lock()
                    .await
                    .downcast_ref::<T>()
                    .expect("That global variable doesn't seem to exist!")
                    .clone()
            })
        })
    }

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
