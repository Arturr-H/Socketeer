use std::net::SocketAddr;

use serde::Serialize;
use serde_json::json;
use tokio_tungstenite::tungstenite::Message;

use crate::peers::{Peers, PeerMap};

/// The server response
pub struct Response {
    /// The content we'll send back to client (Often in JSON format)
    content: String,

    /// To which peers should this request go to?
    to: PeerDirection
}

/// The server response without any direction specified
pub struct EmptyDestinationResponse {
    /// The content we'll send back to client (Often in JSON format)
    content: String,
}

/// This enum decides to which peers a response should be sent to
#[derive(Default)]
pub enum PeerDirection {
    /// Don't send this response to anyone (used in `Response::empty() method`)
    #[default]
    None,

    /// Send this to all peers connected to the server
    All,

    /// Send the response back to only the request origin
    Pong(SocketAddr),

    /// Send the response back to some peers
    Multiple(Vec<SocketAddr>),

    /// Send the reponse back to all peers connected to the server except some
    AllExcept(Vec<SocketAddr>)
}

/* Method implementations */
impl Response {

    /// Construct an empty response, going to nobody, with no content.
    /// If this is used for unimplemented code blocks, use the `unimplemented!()`
    /// or `todo!()` macro instead.
    pub fn empty() -> Self {
        Self { content: String::new(), to: PeerDirection::default() }
    }

    /// Construct a response with json payload status looking something like this:
    /// 
    /// ```json
    /// { "status": 200 }
    /// ```
    pub fn payload_status(status: u16) -> EmptyDestinationResponse {
        EmptyDestinationResponse {
            content: json!({
                "status": status
            }).to_string(),
        }
    }

    /// Respond with json data - any variable that implements `serde::Serialize`
    /// 
    /// ## Example
    /// ```
    /// use serde::Serialize;
    /// 
    /// /* What we'll respond with */
    /// #[derive(Serialize)]
    /// struct MessageResponse {
    ///     status: usize,
    ///     text: String,
    ///     user_id: usize
    /// }
    /// 
    /// /* Endpoint function */
    /// fn some_endpoint(req: Request) -> Response {
    ///     let json = MessageResponse {
    ///         status: 200,
    ///         text: String::from("hello"),
    ///         user_id: 100001,
    ///     };
    ///     
    ///     Response::json(json).to_all()
    /// }
    /// ```
    /// 
    /// Remember that `serde::Serialize` is already implemented for most
    /// types in std - therefore you can pass vectors, strings, number and more
    /// into the `.json(T)` method
    /// 
    /// ## Panics
    /// Serialization can fail if `T`'s implementation of Serialize
    /// decides to fail, or if `T` contains a map with non-string keys.
    pub fn json<T>(data: T) -> EmptyDestinationResponse
    where
        T: Serialize
    {
        EmptyDestinationResponse {
            content: match serde_json::to_string(&data) {
                Ok(e) => e,
                Err(_) => panic!("Can't serialize `T`!")
            },
        }
    }
    
    /// Respond with any type of text.
    /// `data` is any type that implements `Into<String>`.
    /// 
    /// ## Examples
    /// ```
    /// fn some_endpoint(req: Request) -> Response {
    ///     Response::text("Hello, world!").to_all()
    /// }
    /// ```
    pub fn text<T: Into<String>>(data: T) -> EmptyDestinationResponse {
        EmptyDestinationResponse {
            content: data.into()
        }
    }

    /// Private method for this crate. Won't be needed outside of crate
    pub(crate) fn content(&self) -> &String {
        &self.content
    }

    /// Private method for this crate. Respond functionality
    pub(crate) fn respond(self, peer_map: PeerMap) -> () {
        let message = Message::Text(self.content);

        /* Check to which peers we'll send the message to */
        match self.to {
            PeerDirection::None => (),
            PeerDirection::All => {
                for recp in peer_map
                    .iter()
                    .map(|(_, ws_sink)| ws_sink) {
                    recp.unbounded_send(message.clone()).ok();
                };
            },
            PeerDirection::AllExcept(except) => {
                for recp in peer_map
                    .iter()
                    .filter(|(peer_addr, _)| !except.contains(&peer_addr))
                    .map(|(_, ws_sink)| ws_sink) {
                    recp.unbounded_send(message.clone()).ok();
                };
            },
            PeerDirection::Multiple(multiple) => {
                for recp in peer_map
                    .iter()
                    .filter(|(peer_addr, _)| multiple.contains(&peer_addr))
                    .map(|(_, ws_sink)| ws_sink) {
            
                    recp.unbounded_send(
                        message.clone()
                    ).ok();
                };
            },
            PeerDirection::Pong(addr) => {
                for recp in peer_map {
                    if recp.0 == addr {
                        recp.1.unbounded_send(
                            message.clone()
                        ).ok();
                        break;
                    }
                }
            }
        }
    }
}

/* Method impls */
impl EmptyDestinationResponse {

    /// Set the response destination manually
    pub fn to_direction(self, to: PeerDirection) -> Response { Response { content: self.content, to } }

    /// Don't send this response to anyone (used in `Response::empty() method`)
    pub fn to_none(self) -> Response { Response { content: self.content, to: PeerDirection::None } }

    /// Send this to all peers connected to the server
    pub fn to_all(self) -> Response { Response { content: self.content, to: PeerDirection::All } }

    /// Send the response back to only the request origin
    pub fn to_origin(self, origin: SocketAddr) -> Response { Response { content: self.content, to: PeerDirection::Pong(origin) } }

    /// Send the response back to some peers
    pub fn to_selected(self, selected: Vec<SocketAddr>) -> Response { Response { content: self.content, to: PeerDirection::Multiple(selected) } }

    /// Send the reponse back to all peers connected to the server except some
    pub fn to_all_except(self, except: Vec<SocketAddr>) -> Response { Response { content: self.content, to: PeerDirection::AllExcept(except) } }
}
