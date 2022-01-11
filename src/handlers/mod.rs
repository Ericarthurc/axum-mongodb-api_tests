use axum::{http::StatusCode, response::IntoResponse};

pub mod api;

pub async fn handler_404() -> impl IntoResponse {
    println!("404!");
    (StatusCode::NOT_FOUND, "nothing to see here")
}

pub async fn handler_taco() -> impl IntoResponse {
    println!("TACOS!");
    (StatusCode::OK, "nothing to see here... but TACOS!")
}
