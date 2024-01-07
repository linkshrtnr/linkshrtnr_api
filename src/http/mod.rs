use axum::{Extension, Router};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

use crate::routes;
use anyhow::Result;
use tracing::info;
#[derive(Clone)]
pub struct ApiContext {
    pub db: PgPool,
}

pub async fn serve(db: PgPool) -> Result<(), anyhow::Error> {
    let app = router().layer(ServiceBuilder::new().layer(Extension(ApiContext { db })));

    let port = 3000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("router initialized, now listening on port {}", port);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

fn router() -> Router {
    let assets_path = std::env::current_dir().unwrap();
    Router::new()
        .nest("/", routes::web_routes::get_routes())
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .nest("/api", routes::user_routes::get_routes())
        .nest("/api/link", routes::link_routes::get_routes())
        .nest("/api/auth", routes::auth_routes::get_routes())
        .nest("/api/qrcode", routes::qrcode_routes::get_routes())
}
