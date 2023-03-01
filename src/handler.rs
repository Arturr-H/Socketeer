/* Imports */
use crate::{ peers::{Peers, PeerMap}, request::Request, server::EndpointFunction, response::Response };
use std::{ net::SocketAddr, collections::HashMap, any::Any, sync::Arc };
use futures_channel::mpsc::unbounded;
use futures_util::{ future, pin_mut, stream::TryStreamExt, StreamExt };

use serde_derive::Deserialize;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;

/* Request type checker */
#[derive(Deserialize)]
struct RequestType {
    #[serde(rename = "type")]
    _type: String
}

/* Main event handler */
pub async fn handle_connection(
    endpoints: HashMap<String, Box<EndpointFunction>>,
    peers: Peers,
    raw_stream: TcpStream,
    addr: SocketAddr,
) {
    let peer_map = &mut peers.lock().await;
    let ws_stream = match tokio_tungstenite::accept_async(raw_stream).await {
        Ok(e) => e,
        Err(_) => return
    };

    /* Insert to peers */
    let (tx, rx) = unbounded();
    peer_map.insert(addr, tx);
    let (outgoing, incoming) = ws_stream.split();

    /* Message loop */
    let broadcast_incoming = incoming.try_for_each(|data| {
        match &data {
            Message::Text(text) => {
                /* Try parse request type */
                match serde_json::from_str::<RequestType>(text) {
                    Ok(e) => {
                        let peer_map = peer_map.clone();
                        match find_caller(endpoints.clone(), e._type, Request::new(addr, &peer_map, text)) {
                            Some(e) => {
                                e.respond(peer_map);

                                future::ok(())
                            },

                            /* 404 equivalent */
                            None => future::ok(())
                        }
                    },
                    Err(_) => future::ok(())
                }
            },
            _ => future::ok(())
        }
    });


    /* Enable user to also recieve messages */
    let receive_from_others = rx.map(Ok).forward(outgoing);
    pin_mut!(broadcast_incoming, receive_from_others);

    /* Run recv / broadcast */
    future::select(broadcast_incoming, receive_from_others).await;
    
    /* Remove peer */
    peers.lock().await.remove(&addr);
}

/* Find what function to call */
fn find_caller(
    endpoints: HashMap<String, Box<EndpointFunction>>,
    _type: String,
    request_data: Request
) -> Option<Response> {
    for (name, call) in endpoints.iter() {
        if name.to_lowercase() == _type.to_lowercase() {
            return Some(call(request_data));
        }
    }

    None
}
