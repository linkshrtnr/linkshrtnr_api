use crate::http::ApiContext;
use axum::{http::StatusCode, routing::post, Extension, Form, Router};
use serde::Deserialize;
use tracing::info;

#[derive(sqlx::FromRow, Debug, Deserialize)] // Add this line
struct LinkRequest {
    original_url: String,
    path: String,
}
struct LinkResponse {
    link: String,
}

pub fn get_routes() -> Router {
    Router::new().route("/", post(add_link))
}

async fn add_link(ctx: Extension<ApiContext>, Form(payload): Form<LinkRequest>) -> StatusCode {
    //get the link from the form
    info!("{:?}", payload.original_url);
    info!("{:?}", payload.path);
    //store the links in the database
    let link = sqlx::query_as::<_, LinkRequest>(
        "INSERT INTO links (original_url, path) VALUES ($1, $2) RETURNING link",
    )
    .bind(payload.original_url)
    .bind(payload.path)
    .fetch_one(&ctx.db) // Replace sqlx_connection with pool
    .await
    .unwrap();

    //return the link / return text with link and show it in the UI
    StatusCode::OK
}
