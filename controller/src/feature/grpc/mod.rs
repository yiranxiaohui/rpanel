use rpanel_grpc::docker::grpc::DockerReply;
use tokio::sync::mpsc::Sender;
use tonic::transport::Server;
use tracing::info;
use rpanel_grpc::docker::grpc::greeter_server::GreeterServer;
use crate::config::get_config;
use crate::feature::grpc::docker::DockerGreeter;

pub struct Grpc {
    pub tx: Sender<DockerReply>
}

pub mod docker;
mod handle;

pub async fn init_grpc() {
    let config = get_config();
    let addr = format!("127.0.0.1:{}", config.port).parse().expect("Failed to parse socket address");
    let greeter = DockerGreeter::default();
    info!("gRPC server listening on {}", addr);
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await.expect("gRPC server error");
}