/* Imports */
use std::{collections::HashMap, sync::Arc, any::Any};
use tokio::{ net::TcpListener, sync::Mutex };
use crate::{
    peers::Peers,
    request::Request,
    response::Response, handler::handle_connection
};

/* Types */
pub type EndpointFunction = fn(Request) -> Response;

/* Main */
pub struct Server {
    /// _\[REQUIRED\]_ WS address
    address: Option<String>,

    /// _\[REQUIRED\]_ WS port
    port: Option<usize>,

    /// Server endpoints
    endpoints: HashMap<String, Box<EndpointFunction>>,

    /// All the peers (requests) are stored in here
    peers: Peers,

    /// Global data shared between all endpoints
    global: Arc<tokio::sync::Mutex<(dyn Any + Send)>>
}

/* Method impls */
impl Server {
    /// Construct a new server struct
    pub fn new() -> Self {
        Self { address: None, port: None, endpoints: HashMap::new(), peers: Peers::new(), global: Arc::new(Mutex::new(0usize)) }
    }

    /// Create a new websocket endpoint
    pub fn endpoint(&mut self, name: &str, func: EndpointFunction) -> &mut Self {
        self.endpoints.insert(name.to_owned(), Box::new(func));
        self
    }

    /// Set the global data type which will be shared across all endpoints.
    /// `data` has to implement `Send`, and can be accessed via req
    pub fn set_global<Data: Send + 'static>(&mut self, data: Data) -> &mut Self {
        self.global = Arc::new(Mutex::new(data));
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
            tokio::spawn(
                handle_connection(
                    self.global.clone(), 
                    self.endpoints.clone(), 
                    self.peers.clone(),
                    stream, 
                    addr
                )
            );
        }
    }

    /// Get peers
    pub fn peers(&self) -> &Peers {
        &self.peers
    }
}
