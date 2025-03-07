use axum::response::IntoResponse;

pub async fn post_uploads() -> impl IntoResponse {
    tracing::info!("post uploads")
}
