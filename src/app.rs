/// Use axum capabilities.
use axum::routing::get;

/// Create our application by creating our router.
pub fn app() -> axum::Router {
    axum::Router::new()
        .fallback(fallback)
        .route("/uptime", get(uptime))
}

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, uri.to_string())
}

/// Create the constant INSTANT so the program can track its own uptime.
pub static INSTANT: std::sync::LazyLock<std::time::Instant> = std::sync::LazyLock::new(|| std::time::Instant::now());

/// axum handler for "GET /uptime" which shows the program's uptime duration.
/// This shows how to write a handler that uses a global static lazy value.
pub async fn uptime() -> String {
    format!("{}", INSTANT.elapsed().as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test() {
        let app: axum::Router = app();
        let server = TestServer::new(app).unwrap();
        let response_text_0 = server.get("/uptime").await.text();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let response_text_1 = server.get("/uptime").await.text();
        assert!(response_text_0 < response_text_1, "{} < {}", response_text_0, response_text_1);
    }

}

