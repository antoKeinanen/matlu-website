mod error;
mod static_html;
mod auth;

use axum::{Router, routing::get};

use crate::{pages::error::fallback, static_page};

static_page!(index, "Etusivu", "pages/index.html");
static_page!(login_form, "Sisäänkirjautuminen", "pages/login.html");

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index)).fallback(fallback)
        .route("/login", get(login_form)).fallback(fallback)
}
