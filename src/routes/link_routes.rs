use crate::{
    http::ApiContext,
    structs::{LinkRequest, LinkResponse},
};
use axum::{routing::post, Extension, Form, Router};
use rand;
use rand::Rng;
use sqlx::{Error, PgPool, Row};

pub fn get_routes() -> Router {
    Router::new().route("/", post(add_link))
}

async fn add_link(
    ctx: Extension<ApiContext>,
    Form(payload): Form<LinkRequest>,
) -> Result<String, String> {
    // Store the links in the database
    let link = match insert_link(&payload, &ctx.db).await {
        Ok(link) => link,
        Err(err) => {
            eprintln!("Error inserting link: {}", err);
            return Err("<div class=\"p-2 animate-in text-red-900 bg-red-300 border-2 border-red-600\">Something went wrong</div>".to_string());
        }
    };
    //return html

    Ok(format!(
        "<div class=\"p-2 text-green-900 animate-in bg-green-300 border-2 border-green-600\">Your link: <a href=\"http://lurl.es/{}\" target=\"_blank\">https://lurl.es/{}</a></div>",
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
