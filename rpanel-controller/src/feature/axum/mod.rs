mod handle;

use std::net::SocketAddr;
use axum::Router;
use axum::routing::{get, post};
use tower_http::services::{ServeDir, ServeFile};
use tracing::info;
use crate::feature::axum::handle::{get_agent_list, trigger_pull_image, sse_handler};

pub async fn init_axum() {
    let app = Router::new()
        .route("/api/v1/agent/list", get(get_agent_list))
        .route("/api/v1/image/pull", post(trigger_pull_image))
        .route("/api/v1/events", get(sse_handler))
        .fallback_service(
            ServeDir::new("dist")
                .fallback(ServeFile::new("dist/index.html"))
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 5666));
    info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}