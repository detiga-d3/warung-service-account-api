//! # App
//!
//! `app` module contains a setup for [`axum`](axum)'s server.
//!

use axum::{
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use crate::handlers;

/// Generate axum's [`Router`](axum::Router)
pub fn app() -> Router {
    Router::new()
        // List of routes
        .route("/", get(handlers::root))
        // Trace layer to create span for tracing
        .layer(TraceLayer::new_for_http())
}

/// Initialize [`tracing-subscriber`](tracing_subscriber)
pub fn init_tracing() {
    // Init a tracing subscriber
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                // If RUST_LOG env doesn't exist, set this crate's log to the highest level of verbosity (trace)
                .or_else(|_| EnvFilter::try_new("warung_service_account_api=trace,tower_http=trace"))
                .unwrap(),
        )
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn root_works() {
        let app = app();

        let server = TestServer::new(app).unwrap();

        let response = server
            .get("/")
            .await;

        response.assert_status_ok();
        response.assert_text("Lorem ipsum dolor sit amet");
    }
}
