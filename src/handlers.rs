//! # Handlers
//!
//! `handlers` module consists of axum handler.

/// Handler for `GET "/"`
pub async fn root() -> &'static str {
    "Lorem ipsum dolor sit amet"
}
