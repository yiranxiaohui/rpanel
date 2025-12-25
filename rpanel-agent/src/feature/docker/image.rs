use bollard::query_parameters::CreateImageOptions;
use tonic::codegen::tokio_stream::StreamExt;
use tonic::Status;
use tracing::{info, error};
use rpanel_common::docker::PullImageProgress;
use rpanel_grpc::docker::grpc::{Action, DockerRequest};
use crate::feature::docker::get_docker;
use crate::feature::grpc::get_grpc;
use crate::config::get_config;

pub async fn pull_image(image: String) -> Result<(), Status> {
    let docker = get_docker();
    let config = get_config();

    let options = Some(CreateImageOptions {
        from_image: Some(image.clone()),
        ..Default::default()
    });

    info!("Start pulling image: {}", image);

    let mut stream = docker.create_image(options, None, None);

    while let Some(item) = stream.next().await {
        match item {
            Ok(msg) => {
                let progress = PullImageProgress {
                    status: msg.status,
                    progress: msg.progress,
                    id: msg.id,
                };
                
                if let Some(grpc) = get_grpc().await {
                    let mut request = DockerRequest {
                        agent_id: config.id.clone(),
                        payload: serde_json::to_string(&progress).unwrap_or_default(),
                        ..Default::default()
                    };
                    request.set_action(Action::PullImage);
                    grpc.send(request).await;
                }
            }
            Err(e) => {
                error!("Error pulling image {}: {}", image, e);
                // Optionally send error back to controller?
            }
        }
    }

    info!("Image pulled successfully: {}", image);
    Ok(())
}
