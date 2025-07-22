//! # Handlers
//!
//! `handlers` module consists of [`axum`](axum) [`handler`](axum::handler).

/// Handler for `GET "/"`
pub async fn root() -> &'static str {
    tracing::info!("Root is called!");
    "Lorem ipsum dolor sit amet"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn root_handler_return_correctly() {
        let result = root().await;

        assert_eq!(result, "Lorem ipsum dolor sit amet");
    }
}
