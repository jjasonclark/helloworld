#![deny(warnings)]
extern crate config;
extern crate log;
use std::env;
use warp::Filter;

fn load_config() -> Result<config::Config, config::ConfigError> {
    let mut settings = config::Config::new();
    settings.merge(config::File::with_name("config/default"))?;
    settings.merge(config::File::with_name("config/local").required(false))?;
    Ok(settings)
}

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let settings = load_config().unwrap();
    let address = settings.get("server.address").unwrap();
    let port = settings.get("server.port").unwrap();
    let addr = std::net::SocketAddr::new(address, port);

    let warp_logger = warp::log("web");
    let hello_route = warp::get()
        .and(warp::path::end())
        .map(|| warp::reply::html("<html><body>Hello, world!</body></html>"));

    warp::serve(hello_route.with(warp_logger)).run(addr).await;
}
