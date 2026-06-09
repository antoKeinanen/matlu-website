use std::env;

use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

pub(crate) async fn fallback() -> AppError {
    AppError::NotFound("This page does not exist.".to_string())
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    status_code: u16,
    title: String,
    message: String,
    ctx: crate::pages::PageContext,
}

pub enum AppError {
    NotFound(String),
    InternalServerError(String),
    BadRequest(String),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn title(&self) -> &str {
        match self {
            AppError::NotFound(_) => "Not Found",
            AppError::InternalServerError(_) => "Internal Server Error",
            AppError::BadRequest(_) => "Bad Request",
        }
    }

    fn message(&self) -> &str {
        match self {
            AppError::NotFound(msg)
            | AppError::InternalServerError(msg)
            | AppError::BadRequest(msg) => msg,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let template = ErrorTemplate {
            status_code: self.status_code().as_u16(),
            title: self.title().to_string(),
            message: self.message().to_string(),
            ctx: super::PageContext {
                cdn_url: env::var("CDN_URL").unwrap_or_else(|_| "http://localhost:8000".to_string()),
            },
        };

        (self.status_code(), Html(template.render().unwrap())).into_response()
    }
}
