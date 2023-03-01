use std::collections::HashMap;

/* Imports */
use tokio::net::TcpListener;
use crate::{
    peers::Peers,
    request::RequestData,
    response::ServerResponse, handler::handle_connection
};

/* Types */
pub type Response = Box<dyn ServerResponse>;

/* Main */
pub struct Server {
    /// _\[REQUIRED\]_ WS address
    address: Option<String>,

    /// _\[REQUIRED\]_ WS port
    port: Option<usize>,

    /// Server endpoints
    endpoints: HashMap<String, Box<fn(RequestData) -> ()>>,

    /// All the peers (requests) are stored in here
    peers: Peers,
}

/* Method impls */
impl Server {
    /// Construct a new server struct
    pub fn new() -> Self {
        Self { address: None, port: None, endpoints: HashMap::new(), peers: Peers::new() }
    }

    /// Create a new websocket endpoint
    pub fn endpoint(&mut self, name: &str, func: fn(RequestData) -> ()) -> &mut Self {
        self.endpoints.insert(name.to_owned(), Box::new(func));
        self
    }

    /// Start the server
    pub async fn start(&mut self) -> () {
        /* Initialize stream */
        let addr = format!("{}:{}", "127.0.0.1", 8080);
        let server = TcpListener::bind(&addr).await.unwrap();

        /* Create peer map and games */
        self.peers = Peers::new();

        /* Incoming requests */
        while let Ok((stream, addr)) = server.accept().await {
            tokio::spawn(handle_connection(self.endpoints.clone(), self.peers.clone(), stream, addr));
        }
    }

    /// Get peers
    pub fn peers(&self) -> &Peers {
        &self.peers
    }
}
