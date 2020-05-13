#![deny(warnings)]
extern crate log;
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let warp_logger = warp::log("web");
    let hello_route = warp::get()
        .and(warp::path::end())
        .map(|| warp::reply::html("<html><body>Hello, world!</body></html>"));

    warp::serve(hello_route.with(warp_logger))
        .run(([127, 0, 0, 1], 3000))
        .await;
}
