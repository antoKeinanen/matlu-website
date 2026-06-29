use axum::{
    Form, Json, RequestPartsExt, Router,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, NoContent, Response},
    routing::get,
};

use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::LazyLock;
use std::fmt::Display;

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

#[derive(Deserialize)]
pub(crate) struct Login {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize, // Expiry time of the token
    pub iat: usize, // Issued at time of the token
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = "lol".to_string();
    Keys::new(secret.as_bytes())
});

pub async fn login(Form(login_payload): Form<Login>) -> Result<Json<AuthBody>, AuthError> {
    if login_payload.username.is_empty() || login_payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    // Here you can check the user credentials from a database
    if login_payload.username != "foo" || login_payload.password != "bar" {
        return Err(AuthError::WrongCredentials);
    }
    let claims = Claims {
        exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
        username: login_payload.username,
    };

    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "username: {}", self.username)
    }
}
