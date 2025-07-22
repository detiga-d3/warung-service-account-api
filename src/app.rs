//! # App
//!
//! `app` module contains a setup for [`axum`](axum)'s server.
//!

use crate::handlers;
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

/// Generate axum's [`Router`](axum::Router)
pub fn router() -> Router {
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
                // If RUST_LOG env doesn't exist, set this crate's and tower-http log to the highest level of verbosity (trace)
                .or_else(|_| {
                    EnvFilter::try_new("warung_service_account_api=trace,tower_http=trace")
                })
                .unwrap(),
        )
        .init();
}
