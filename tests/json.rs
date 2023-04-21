use std::convert::Infallible;

use warp::Filter;

extern crate as_reply;

#[derive(serde::Serialize, as_reply::AsJsonReply)]
struct JsonTest {}

pub fn handler() -> impl Filter<Extract = (impl warp::Reply,), Error = Infallible> + Clone {
    warp::any().map(|| JsonTest {})
}
