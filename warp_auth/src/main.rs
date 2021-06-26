use std::sync::Arc;
use std::collections::HashMap;
use serde::Deserialize; // A framework for serializing and deserializing Rust data structures.
use tokio::sync::Mutex; // Synchronization primitives for use in asynchronous contexts from tokio, an event-driven, non-blocking I/O platform for writing asynchronous applications.
use warp::Filter;

#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    let port = 3030;

    let db = Arc::new(Mutex::new(HashMap::<String, User>::new()));
    let db = warp::any().map(move || Arc::clone(&db));  

    // Each incoming request passes through a chain of Filters which can either do something with that request or reject it:
    // let routes = warp::any().map(|| "Hello, World!");
    let register = warp::path("register").map(|| "Register");
    let login = warp::path("login").map(|| "Log in");
    let logout = warp::path("logout").map(|| "Log out");
    
    let routes = register.or(login).or(logout);
    let routes = warp::path("api").and(routes);

    println!("🚀 Warp server running on localhost:{}", port);
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}