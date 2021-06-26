use std::sync::Arc;
use std::collections::HashMap;
use serde::Deserialize; // A framework for serializing and deserializing Rust data structures.
use tokio::sync::Mutex; // Synchronization primitives for use in asynchronous contexts from tokio, an event-driven, non-blocking I/O platform for writing asynchronous applications.
use warp::Filter;
use warp::http::StatusCode;
use argon2::{self, Config};
use rand::Rng;

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
    
    let routes = register.or(login);
    let routes = warp::path("api").and(routes);

    println!("🚀 Warp server running on localhost:{}", port);
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}

pub async fn register(
    new_user: User,
    db: Arc<Mutex<HashMap<String, User>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut users = db.lock().await;
    if users.contains_key(&new_user.username) {
        return Ok(StatusCode::BAD_REQUEST);
    }
    let hashed_user = User {
        username: new_user.username,
        password: hash_password(new_user.password.as_bytes()),
    }
    users.insert(hashed_user.username.clone(), hashed_user);
    Ok(StatusCode::CREATED)
}

pub async fn login(
    credentials: User,
    db: Arc<Mutex<HashMap<String, User>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let users = db.lock().await;
    match users.get(&credentials.username) {
        None => Ok(StatusCode::BAD_REQUEST),
        Some(user) => {
            if verify_password(&user.password, credentials.password.as_bytes()) {
                Ok(StatusCode::OK)
            } else {
                Ok(StatusCode::UNAUTHORIZED)
            }
        }
    }
}

pub fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn verify_password(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}