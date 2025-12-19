mod feature;
mod config;

use tonic::transport::Server;
use tracing::info;
use rpanel_grpc::docker::grpc::greeter_server::GreeterServer;
use crate::feature::grpc::docker::DockerGreeter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let addr = "[::1]:5666".parse()?;
    let greeter = DockerGreeter::default();

    info!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}