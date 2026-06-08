mod error;
mod static_html;

use axum::{Router, routing::get};

use crate::{pages::error::fallback, static_page};

static_page!(index, "Etusivu", "pages/index.html");

pub fn get_router() -> Router<AppState> {
    Router::new().route("/", get(index)).fallback(fallback)
}
