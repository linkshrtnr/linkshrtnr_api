use crate::http::ApiContext;
use axum::{http::StatusCode, routing::post, Extension, Form, Router};
use rand;
use rand::Rng;
use serde::Deserialize;
use sqlx::{Error, PgPool};
use tracing::info;

#[derive(sqlx::FromRow, Debug, Deserialize)] // Add this line
struct LinkRequest {
    original_url: String,
    path: String,
}

pub fn get_routes() -> Router {
    Router::new().route("/", post(add_link))
}

async fn add_link(ctx: Extension<ApiContext>, Form(payload): Form<LinkRequest>) -> StatusCode {
    // Get the link from the form
    info!("{:?}", payload.original_url);
    info!("{:?}", payload.path);

    // Store the links in the database
    if let Err(err) = insert_link(&payload, &ctx.db).await {
        // Handle the error, log it, and return an appropriate status code
        eprintln!("Error inserting link: {}", err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    // Return the link / return text with link and show it in the UI
    StatusCode::OK
}
async fn insert_link(payload: &LinkRequest, pool: &PgPool) -> Result<(), Error> {
    let path = if payload.path.is_empty() {
        generate_random_string(5)
    } else {
        payload.path.clone()
    };

    let _insertion =
        sqlx::query("INSERT INTO \"Links\" (original_url, short_url) VALUES ($1, $2);")
            .bind(&payload.original_url)
            .bind(&path)
            .execute(pool)
            .await?;

    Ok(())
}
fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen_range('a'..='z')).collect()
}
