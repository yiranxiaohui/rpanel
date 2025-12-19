use bollard::query_parameters::CreateImageOptions;
use tonic::codegen::tokio_stream::StreamExt;
use tonic::Status;
use tracing::{info};
use crate::feature::docker::get_docker;
use crate::feature::grpc::get_grpc;

pub async fn pull_image(image: String) -> Result<(), Status> {
    // let docker = get_docker();
    // 
    // let options = Some(CreateImageOptions {
    //     from_image: Some(image.clone()),
    //     ..Default::default()
    // });
    // 
    // let mut stream = docker.create_image(options, None, None);
    // 
    // while let Some(item) = stream.next().await {
    //     let msg = item.map_err(|e| Status::internal(e.to_string()))?;
    //     let payload = Payload::PullImageRequest(PullImageRequest {
    //         status: msg.status,
    //         progress: msg.progress,
    //     });
    //     let mut request = DockerRequest {
    //         agent_id: "".to_string(),
    //         payload: Some(payload),
    //         ..Default::default()
    //     };
    //     request.set_action(Action::PullImage);
    //     let grpc = get_grpc().await.unwrap();
    //     grpc.send(request).await;
    // }
    // 
    // info!("image pulled: {}", image);
    Ok(())
}