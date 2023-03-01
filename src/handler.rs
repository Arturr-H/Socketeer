/* Imports */
use crate::{ peers::Peers, request::Request, server::EndpointFunction };
use std::{net::SocketAddr, collections::HashMap};
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
    addr: SocketAddr
) {
    let ws_stream = match tokio_tungstenite::accept_async(raw_stream).await {
        Ok(e) => e,
        Err(_) => return
    };

    /* Insert to peers */
    let (tx, rx) = unbounded();
    match peers.lock() { Ok(e) => e, Err(_) => return }.insert(addr, tx);
    let (outgoing, incoming) = ws_stream.split();

    /* Message loop */
    let broadcast_incoming = incoming.try_for_each(|data| {
        let peers = match peers.lock() {
            Ok(e) => e,
            Err(_) => return future::ok(())
        };

        match &data {
            Message::Text(text) => {

                /* Try parse request type */
                match serde_json::from_str::<RequestType>(text) {
                    Ok(e) => {
                        find_caller(endpoints.clone(), e._type, Request::new(addr, peers.clone(), text));
                        return future::ok(())
                    },
                    Err(_) => return future::ok(())
                };
            },
            _ => return future::ok(())
        };
    });


    /* Enable user to also recieve messages */
    let receive_from_others = rx.map(Ok).forward(outgoing);
    pin_mut!(broadcast_incoming, receive_from_others);

    /* Run recv / broadcast */
    future::select(broadcast_incoming, receive_from_others).await;
    
    /* Remove peer */
    match peers.lock() {
        Ok(e) => e,
        Err(_) => return
    }.remove(&addr);
}

/* Find what function to call */
fn find_caller(
    endpoints: HashMap<String, Box<EndpointFunction>>,
    _type: String,
    request_data: Request
) -> () {
    for (name, call) in endpoints.iter() {
        if name.to_lowercase() == _type.to_lowercase() {
            call(request_data);
            break;
        }
    }
}
