mod bridge;
mod handle;

use tonic::transport::Server;
use tracing::info;
use grpc::bridge::grpc::bridge_service_server::BridgeServiceServer;
use crate::config::get_config;
use crate::feature::grpc::bridge::BridgeGreeter;

pub async fn init_grpc() {
    let config = get_config();
    let addr = format!("127.0.0.1:{}", config.port).parse().expect("Failed to parse socket address");
    let greeter = BridgeGreeter::default();
    info!("gRPC server listening on {}", addr);
    Server::builder()
        .add_service(BridgeServiceServer::new(greeter))
        .serve(addr)
        .await.expect("gRPC server error");
}