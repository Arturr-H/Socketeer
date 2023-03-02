# Socketeer

Websocket API built in rust on top of [*tokio-tungstenite*](https://github.com/snapview/tokio-tungstenite). I created this project because I wanted a quick way of prototyping websocket applications, instead of having to write a lot of boilerplate code.

## Examples
#### `Server side`
```rust
#[tokio::main]
async fn main() -> () {
    server::Server::new()
        .endpoint("some_endpoint", some_endpoint)
        .set_global::<u8>(0)
        .start().await;
}

fn some_endpoint(req: Request) -> Response {
    /* Increase the global data */
    req.global_mut::<u8>(|v| {
        *v += 1
    });

    /* Respond with the global data */
    Response::json(&req.global::<u8>()).to_all()
}
```

#### `Client side`
```javascript
/* Open the websocket connection */
const ws = new Websocket("ws://localhost:8080/");

ws.onopen = (event) => {
    ws.send(JSON.stringify({
        /* We want to reach the endpoint
            called "some_endpoint" */
        "type": "some_endpoint"
    }));
};

ws.onmessage = (data) => {
    const data = JSON.parse(data);
    console.log(data); /// Logs the current count
}
```

## The `Response` struct
The response struct is intended to simplify sending data to clients. Here are some examples:

```rust
fn endpoint(req: Request) -> Response {
    #[derive(Serialize)]
    struct Data {
        value: usize,
        other: String
    }

    let data = Data { value: 1, other: String::from("Hello!") };

    Response::json(data).to_all()
}
```

Notice the `.to_all()` method at the end of the code-block above. Once you've selected the data you want to send, you can specify which clients you want to send the data to!

There are many options to choose from when deciding which clients should recieve the data, here are some examples:

```rust
/* Send to all clients */
Response::text("Hello, world!").to_all();

/*  Send to all clients except some.
    Useful for eg sending messages to
    all users except the sender. */
Response::text("Hello, world!").to_all_except(vec![ /* ... */ ]);

/* Send to some selected clients */
Response::text("Hello, world!").to_selected(vec![ /* ... */ ]);

/*  Send to one origin. Useful for
    sending eg error messages to
    one user or personal data */
Response::text("Hello, world!").to_origin(/* ... */);
```