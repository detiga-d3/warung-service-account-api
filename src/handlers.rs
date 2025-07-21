//! # Handlers
//!
//! `handlers` module consists of [`axum`](axum) [`handler`](axum::handler).

/// Handler for `GET "/"`
pub async fn root() -> &'static str {
    tracing::info!("Root is called!");
    "Lorem ipsum dolor sit amet"
}
