mod handle;

use std::net::SocketAddr;
use axum::Router;
use axum::routing::{get, post};
use tower_http::services::{ServeDir, ServeFile};
use tracing::info;
use crate::feature::axum::handle::{
    get_agent_list, trigger_pull_image, sse_handler,
    get_docker_containers, get_docker_images, trigger_refresh_docker,
    trigger_start_container, trigger_stop_container, trigger_remove_container,
    trigger_remove_image, trigger_run_container
};

pub async fn init_axum() {
    let app = Router::new()
        .route("/api/v1/agent/list", get(get_agent_list))
        .route("/api/v1/image/pull", post(trigger_pull_image))
        .route("/api/v1/events", get(sse_handler))
        .route("/api/v1/agent/{id}/docker/containers", get(get_docker_containers))
        .route("/api/v1/agent/{id}/docker/images", get(get_docker_images))
        .route("/api/v1/agent/{id}/docker/refresh", post(trigger_refresh_docker))
        .route("/api/v1/agent/{id}/docker/container/start", post(trigger_start_container))
        .route("/api/v1/agent/{id}/docker/container/stop", post(trigger_stop_container))
        .route("/api/v1/agent/{id}/docker/container/remove", post(trigger_remove_container))
        .route("/api/v1/agent/{id}/docker/image/remove", post(trigger_remove_image))
        .route("/api/v1/agent/{id}/docker/run", post(trigger_run_container))
        .fallback_service(
            ServeDir::new("dist")
                .fallback(ServeFile::new("dist/index.html"))
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 5666));
    info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}