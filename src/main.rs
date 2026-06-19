mod user;

use axum::{Json, Router, debug_handler, routing::get, routing::post};
use std::net::SocketAddr;
use user::CreateUserRequest;
use user::User;
use user::UserResponse;

#[debug_handler]
async fn get_user() -> Json<UserResponse> {
    println!("[get-user-handler] Request recieved");
    let user = User::new("Test".to_string(), "test@email.com".to_string());

    return Json(UserResponse {
        name: user.get_name().to_string(),
        email: user.get_email().to_string(),
    });
}

async fn create_user(Json(payload): Json<CreateUserRequest>) -> Json<UserResponse> {
    println!("[create-user-handler] Request received: {}", payload.name);
    let user = User::new(payload.name, payload.email);

    return Json(UserResponse {
        name: user.get_name().to_string(),
        email: user.get_email().to_string(),
    });
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", get(get_user))
        .route("/users", post(create_user));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
