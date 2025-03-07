use axum::response::IntoResponse;

pub async fn post_download() -> impl IntoResponse {
    tracing::info!("download handler")
}
