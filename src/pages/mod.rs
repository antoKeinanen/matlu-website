mod error;
mod static_page;

use axum::{Router, routing::get};

use crate::{AppState, pages::error::fallback, static_markdown};

static_markdown!(index, "Etusivu", "pages/index.md");

pub fn get_router() -> Router<AppState> {
    Router::new().route("/", get(index)).fallback(fallback)
}
