use std::net::SocketAddr;
use axum::Router;
use axum::routing::get;

pub async fn init_axum() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum!" }));
    let addr = SocketAddr::from(([127, 0, 0, 1], 5666));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}