# as_warp_reply
Macros for implementing the `warp::reply::Reply` trait on custom structs. This enables you to use your custom structs as responses to requests. For example you can do this:

```rust
#[derive(serde::Serialize, as_warp_reply::JsonReply)]
struct MyCustomResponse {}

pub fn handler() -> impl Filter<Extract = (impl warp::Reply,), Error = Infallible> + Clone {
    warp::any().map(|| MyCustomResponse {})
}

```

## How does it work?
The macro generates code that uses serde to serialize the struct. The expanded macro from the above example would look like this:

```rust
impl warp::reply::Reply for MyCustomResponse {
    fn into_response(self) -> warp::reply::Response {
        match serde_json::to_string(&self) {
            Ok(v) => warp::reply::with_status(v, warp::http::StatusCode::OK).into_response(),
            Err(err) => {
                log::error!("could not serialize response: {}", err);
                warp::reply::with_status("Internal server error", warp::http::StatusCode::INTERNAL_SERVER_ERROR)
                    .into_response()
            }
        }
    }
}
```

As you can see in case of success, the response consists of the serialized struct and sends status code OK. In case of any serialization error, the response consists of the "Internal server error" message and sends status code INTERNAL_SERVER_ERROR. Also the `log` crate is used to log the serialization error.