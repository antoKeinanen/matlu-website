mod error;
mod static_page;

use axum::{Router, routing::get};

use crate::{AppState, pages::error::fallback, static_markdown};

#[derive(Debug, Clone)]
pub struct PageContext {
    pub cdn_url: String,
}

static_markdown!(index, "Etusivu", "pages/index.md");
static_markdown!(members, "Jäsenet", "pages/members.md");

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/members", get(members))
        .fallback(fallback)
}
