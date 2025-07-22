use dotenvy::dotenv;
use std::env;
use tracing::info;
use warung_service_account_api::app::{init_tracing, router};

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Init tracing
    init_tracing();

    // Init app
    let app = router();

    // Construct the address
    let host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("APP_PORT").unwrap_or("3000".to_string());
    let address = format!("{host}:{port}");

    // Start the server
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    info!("Starting server on http://{address}");
    axum::serve(listener, app).await.unwrap();
}
