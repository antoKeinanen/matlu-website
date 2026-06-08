use tower_service::Service;
use worker::*;

use crate::env::Env as DotEnv;

mod env;
mod pages;

#[derive(Clone)]
pub struct AppState {
    pub env: DotEnv,
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    let app_state = AppState {
        env: DotEnv::load(),
    };

    Ok(pages::get_router().with_state(app_state).call(req).await?)
}
