use rpanel_common::docker::PullImageRequest;
use rpanel_grpc::docker::grpc::{Action, DockerReply};
use tracing::error;
use crate::feature::docker::image::pull_image;

pub async fn handle_message(reply: DockerReply) {
    let action = reply.action();
    match action {
        Action::UploadStatus => {}
        Action::CreateContainer => {},
        Action::PullImage => {
            if let Ok(req) = serde_json::from_str::<PullImageRequest>(&reply.payload) {
                // Run in background to not block the receiver loop
                tokio::spawn(async move {
                    if let Err(e) = pull_image(req.image).await {
                        error!("Failed to pull image: {}", e);
                    }
                });
            } else {
                error!("Failed to parse PullImageRequest payload");
            }
        }
        _ => {}
    }
}
