use std::convert::Infallible;

use warp::Filter;

extern crate as_warp_reply;

#[derive(serde::Serialize, as_warp_reply::JsonReply)]
struct JsonTest {}

pub fn handler() -> impl Filter<Extract = (impl warp::Reply,), Error = Infallible> + Clone {
    warp::any().map(|| JsonTest {})
}

#[test]
fn test_compile_json() {
    handler();
}
