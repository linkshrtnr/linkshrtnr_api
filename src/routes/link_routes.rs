use axum::{Router, routing::post};

pub fn get_routes() -> Router {
    Router::new()
        .route("/:id", post(link_handler))
}
async fn link_handler() {
    //get the link from the form
    //store the links in the database
}
