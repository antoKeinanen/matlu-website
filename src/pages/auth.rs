
use axum::{Router, response::NoContent, routing::get};

use axum::Form;
use serde::Deserialize;

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

async fn login(Form(login): Form<Login>) {
    // ...
}

