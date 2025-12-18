use tonic::transport::Server;
use rpanel_grpc::docker::grpc::greeter_server::GreeterServer;
use rpanel_grpc::docker::DockerGreeter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = DockerGreeter::default();

    println!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}