#[macro_use]
extern crate diesel;

mod model;
mod errors;
mod db_access;
mod schema;

use std::env;
use warp::{Filter};
use log::{info};

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let routes = warp::path!("api").map(|| "Hello, World!".to_string());

    info!("Starting server on port 3030");

    warp::serve(routes).run(([127, 0,  0, 1], 3030)).await;
}