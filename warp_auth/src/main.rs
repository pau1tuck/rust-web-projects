use std::sync::Arc;
use std::collections::HashMap;
use serde::Deserialize; // A framework for serializing and deserializing Rust data structures.
use tokio::sync::Mutex; // Synchronization primitives for use in asynchronous contexts from tokio, an event-driven, non-blocking I/O platform for writing asynchronous applications.
use warp::Filter;
use warp::http::StatusCode;

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
    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(db.clone())
        .and_then(register);
    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(db.clone())
        .and_then(login);
    let logout = warp::path("logout").map(|| "Log out");
    
    let routes = register.or(login).or(logout);
    let routes = warp::path("api").and(routes);

    println!("🚀 Warp server running on localhost:{}", port);
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}

async fn register(
    new_user: User,
    db: Arc<Mutex<HashMap<String, User>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut users = db.lock().await;
    if users.contains_key(&new_user.username) {
        return Ok(StatusCode::BAD_REQUEST);
    }
    users.insert(new_user.username.clone(), new_user);
    Ok(StatusCode::CREATED)
}

async fn login(
    credentials: User,
    db: Arc<Mutex<HashMap<String, User>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let users = db.lock().await;
    match users.get(&credentials.username) {
        None => Ok(StatusCode::BAD_REQUEST),
        Some(user) => {
            if credentials.password == user.password {
                Ok(StatusCode::OK)
            } else {
                Ok(StatusCode::UNAUTHORIZED)
            }
        }
    }
}