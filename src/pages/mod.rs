mod error;
mod static_html;

use axum::{Router, handler::HandlerWithoutStateExt, routing::get};
use tower_http::services::ServeDir;

use crate::{pages::error::fallback, static_page};

static_page!(index, "Etusivu", "pages/index.html");

pub fn get_router() -> Router {
    Router::new()
        .nest_service(
            "/static",
            ServeDir::new("assets").not_found_service(fallback.into_service()),
        )
        .route("/", get(index))
        .fallback(fallback)
}
