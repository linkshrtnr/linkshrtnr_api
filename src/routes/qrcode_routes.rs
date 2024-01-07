use axum::{http::StatusCode, routing::post, Form, Router};
use qrcode::render::svg;
use qrcode::QrCode;
use serde::Deserialize;
#[derive(sqlx::FromRow, Debug, Deserialize)] // Add this line
struct QRCodeRequest {
    original_url: String,
}
#[derive(sqlx::FromRow, Debug, Deserialize)] // Add this line
struct QRCodeResponse {
    _short_url: String,
}
pub fn get_routes() -> Router {
    Router::new().route("/", post(add_qrcode))
}

async fn add_qrcode(
    // ctx: Extension<ApiContext>,
    Form(payload): Form<QRCodeRequest>,
) -> Result<String, StatusCode> {
    Ok(generate_qr_from_qrcode(&payload.original_url))
}
fn generate_qr_from_qrcode(url: &str) -> String {
    if let Ok(qr) = QrCode::with_error_correction_level(&url, qrcode::EcLevel::L) {
        let svg = qr
            .render()
            .min_dimensions(100, 100)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();

        svg
    } else {
        // should never (only on very huge codes) happen.
        "".to_string()
    }
}
