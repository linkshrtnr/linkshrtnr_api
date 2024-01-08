use crate::{
    http::ApiContext,
    structs::{LinkRequest, LinkResponse},
};
use axum::{routing::post, Extension, Form, Router};
use rand;
use rand::Rng;
use serde::Deserialize;
use sqlx::{Error, PgPool, Row};

pub fn get_routes() -> Router {
    Router::new().route("/", post(add_link))
}

async fn add_link(
    ctx: Extension<ApiContext>,
    Form(payload): Form<LinkRequest>,
) -> Result<String, String> {
    if payload.original_url.is_empty() {
        return Err("<div class=\"p-2 animate-in text-red-900 bg-red-300 border-2 border-red-600\">Please enter an URL</div>".to_string());
    }
    if payload.path.len() > 0 && payload.path.len() < 5 {
        return Err("<div class=\"p-2 animate-in text-red-900 bg-red-300 border-2 border-red-600\">Path must be at least 5 characters long</div>".to_string());
    }
    if payload.original_url.len() > 200 {
        return Err("<div class=\"p-2 animate-in text-red-900 bg-red-300 border-2 border-red-600\">URL must be less than 200 characters long</div>".to_string());
    }
    if !payload.original_url.starts_with("http://") || !payload.original_url.starts_with("https://")
    {
        return Err("<div class=\"p-2 animate-in text-red-900 bg-red-300 border-2 border-red-600\">URL must start with http:// or https://</div>".to_string());
    }
    // Store the links in the database
    let link = match insert_link(&payload, &ctx.db).await {
        Ok(link) => link,
        Err(err) => {
            eprintln!("Error inserting link: {}", err);
            return Err("<div class=\"p-2 animate-in text-red-900 bg-red-300 border-2 border-red-600\">Something went wrong. Try a different path</div>".to_string());
        }
    };
    //return html

    Ok(format!(
        "<div class=\"p-2 text-green-900 animate-in bg-green-300 border-2 border-green-600\">Your link: <a href=\"https://lurl.es/{}\" target=\"_blank\">https://lurl.es/{}</a></div>",
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
        "INSERT INTO links (original_url, short_url) VALUES ($1, $2) RETURNING id, short_url ",
    )
    .bind(&payload.original_url)
    .bind(&path)
    .fetch_one(pool)
    .await?;
    sqlx::query("INSERT INTO linkclicks (LinkID,ClickCount) VALUES ($1, 0)")
        .bind(insertion.try_get::<i32, _>("id")?)
        .execute(pool)
        .await?;
    let link = LinkResponse {
        short_url: insertion.try_get::<String, _>("short_url")?,
    };
    Ok(link)
}
fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen_range('a'..='z')).collect()
}
