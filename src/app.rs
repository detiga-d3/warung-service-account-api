//! # App
//!
//! `app` module contains a setup for axum's server.
//!

use axum::{
    routing::get,
    Router,
};
use crate::handlers;

/// Generate axum's [Router](axum::Router)
pub fn app() -> Router {
    Router::new()
        .route("/", get(handlers::root))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn root_works() {
        let app = app();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Get the request.
        let response = server
            .get("/")
            .await;

        // Assertions.
        response.assert_status_ok();
        response.assert_text("Lorem ipsum dolor sit amet");
    }
}
