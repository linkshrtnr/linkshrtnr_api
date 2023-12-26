use axum::{
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub fn get_routes() -> Router {
    Router::new()
        .route("/users", get(get_user))
        .route("/users", post(create_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
}
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
async fn get_user() -> &'static str {
    "Hello, from github actions!"
}
async fn update_user() -> &'static str {
    todo!()
}
async fn delete_user() -> &'static str {
    todo!()
}
// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
