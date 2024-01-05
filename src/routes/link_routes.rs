use crate::http::ApiContext;
use axum::{http::StatusCode, routing::post, Extension, Form, Router};
use rand;
use rand::Rng;
use serde::Deserialize;
use sqlx::{Error, PgPool, Row};

#[derive(sqlx::FromRow, Debug, Deserialize)] // Add this line
struct LinkRequest {
    original_url: String,
    path: String,
}
#[derive(sqlx::FromRow, Debug, Deserialize)] // Add this line
struct LinkResponse {
    short_url: String,
}
pub fn get_routes() -> Router {
    Router::new().route("/", post(add_link))
}

async fn add_link(
    ctx: Extension<ApiContext>,
    Form(payload): Form<LinkRequest>,
) -> Result<String, StatusCode> {
    // Store the links in the database
    let link = match insert_link(&payload, &ctx.db).await {
        Ok(link) => link,
        Err(err) => {
            eprintln!("Error inserting link: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    //return html

    Ok(format!(
        "<div>Your link: <a href=\"http://lurl.es/{}\" target=\"_blank\">https://lurl.es/{}</a></div>",
        link.short_url, link.short_url
    ))
}
async fn insert_link(payload: &LinkRequest, pool: &PgPool) -> Result<LinkResponse, Error> {
    let path = if payload.path.is_empty() {
        generate_random_string(5)
    } else {
        payload.path.clone()
    };

    let insertion = sqlx::query(
        "INSERT INTO links (original_url, short_url) VALUES ($1, $2) RETURNING short_url",
    )
    .bind(&payload.original_url)
    .bind(&path)
    .fetch_one(pool)
    .await?;
    let link = LinkResponse {
        short_url: insertion.try_get("short_url")?,
    };
    Ok(link)
}
fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen_range('a'..='z')).collect()
}
