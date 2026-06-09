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
static_markdown!(interests, "Edunvalvonta", "pages/interests.md");
static_markdown!(officials, "Virat", "pages/officials.md");
static_markdown!(rules, "Säännöt", "pages/rules.md");
static_markdown!(faq, "UKK", "pages/faq.md");

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/members", get(members))
        .route("/interests", get(interests))
        .route("/officials", get(officials))
        .route("/rules", get(rules))
        .route("/faq", get(faq))
        .fallback(fallback)
}
