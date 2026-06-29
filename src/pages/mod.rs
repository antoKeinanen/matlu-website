mod auth;
mod error;
mod static_page;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{AppState, pages::error::fallback, static_html, static_markdown};
use auth::{AuthError, Claims, login};

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

static_markdown!(
    matlu_privacy_policy,
    "Matlun tietosuojaseloste",
    "pages/documents/matlu-privacy-policy.md"
);
static_markdown!(
    safer_space_guideline,
    "Turvallisemman tilan periaatteet",
    "pages/documents/safer-space-guideline.md"
);
static_markdown!(
    equality_plan,
    "Yhdenvertaisuussuunnitelma",
    "pages/documents/equality-plan.md"
);
static_markdown!(
    privacy_policy,
    "Tietosuojapolitiikka",
    "pages/documents/privacy-policy.md"
);

static_html!(login_form, "Sisäänkirjautuminen", "pages/login.html");

async fn protected(claims: Claims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{claims}",
    ))
}

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/members", get(members))
        .route("/interests", get(interests))
        .route("/officials", get(officials))
        .route("/rules", get(rules))
        .route("/faq", get(faq))
        .route("/documents/matlu-privacy-policy", get(matlu_privacy_policy))
        .route(
            "/documents/safer-space-guideline",
            get(safer_space_guideline),
        )
        .route("/documents/equality-plan", get(equality_plan))
        .route("/documents/privacy-policy", get(privacy_policy))
        .route("/login", get(login_form))
        .route("/login", post(login))
        .route("/protected", get(protected))
        .fallback(fallback)
}
