#![allow(unused)] // For Beginning
use axum::{
    Router,
    http::Result,
    response::IntoResponse,
    routing::{get, post},
};
use tracing::Subscriber;
use tracing_subscriber::fmt::format::FmtSpan;

mod api;

#[tokio::main]
async fn main() {
    // Logging / tracing
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        // .json()
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");

    // Routes
    let app = Router::new()
        .route("/api/uploads", post(api::upload::post_uploads))
        .route("/api/download/:id", post(api::download::post_download));

    // Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
