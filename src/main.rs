mod routes;
use anyhow::Context;
use axum::Router;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Debug
    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("Initialized router!");
    let port = 3000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    // build our application with a route
    let app = Router::new()
        .nest("/", routes::web_routes::get_routes())
        .nest("/api", routes::user_routes::get_routes());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("router initialized, now listening on port {}", port);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
