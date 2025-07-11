//! # Web service uptime axum
//! 
//! **[documentation](https://docs.rs/web-service-uptime-axum/)**
//! •
//! **[source](https://github.com/joelparkerhenderson/web-service-uptime-axum/)**
//! •
//! **[llms.txt](https://raw.githubusercontent.com/joelparkerhenderson/web-service-uptime-axum/refs/heads/main/llms.txt)**
//! •
//! **[crate](https://crates.io/crates/web-service-uptime-axum)**
//! •
//! **[email](mailto:joel@joelparkerhenderson.com)**
//!
//! Web service that displays the program uptime by using Axum, Tokio, Rust.
//! 
//! This is a very simple web service that we use for testing our systems.
//! 
//! ## Steps
//! 
//! Run the service on host 0.0.0.0 port 3000 or wherever you wish:
//! 
//! ```sh
//! cargo run -- "0.0.0.0:3000"
//! ```
//! 
//! Browse <https://localhost:3000>
//! 
//! You should see a web page that displays the uptime in seconds.
//! 
//! Wait a little bit, then use your browser to reload the web page.
//! 
//! You should see the uptime increase a little bit.
//! 
//! ## References
//! 
//! Based on Demo Rust Axum free open source software:
//! <https://github.com/joelparkerhenderson/demo-rust-axum>
//! 

mod app;

/// Use tracing crates for application-level tracing output.
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Create the constant INSTANT so the program can track its own uptime.
pub static INSTANT: std::sync::LazyLock<std::time::Instant> = std::sync::LazyLock::new(|| std::time::Instant::now());

/// The main function does these steps: 
/// - Start tracing and emit a tracing event.
/// - Get a command line argument as our bind address.
/// - Create our application which is an axum router.
/// - Run our application as a hyper server.
#[tokio::main]  
async fn main() {
    // Start tracing and emit a tracing event.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::event!(tracing::Level::INFO, "main");

    // Get command line arguments.
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Use the first arg for tokio::net::TcpListener::bind(…)  
    let bind_address = match args.get(0) {
        Some(x) => x.clone(),
        None => "0.0.0.0:3000".into(),
    };

    // Create our application which is an axum router.
    let app = crate::app::app();

    // Run our application as a hyper server.
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Shutdown signal to run axum with graceful shutdown when
/// a user presses Ctrl+C or Unix sends a terminate signal.
pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
