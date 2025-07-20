mod handlers;
mod app;

use app::app;
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    // Load .env folder
    dotenv().ok();

    // Init app
    let app = app();

    // Construct the address
    let host = env::var("APP_HOST").unwrap_or("localhost".to_string());
    let port = env::var("APP_PORT").unwrap_or("3000".to_string());
    let address = format!("{host}:{port}");


    // Start the server
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("Server running at http://{address}");
    axum::serve(listener, app).await.unwrap();
}
