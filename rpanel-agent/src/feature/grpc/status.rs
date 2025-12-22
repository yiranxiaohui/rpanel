use rpanel_common::status::{get_system_status, Status};
use rpanel_grpc::docker::grpc::{Action, DockerRequest};
use crate::config::get_config;
use crate::feature::grpc::get_grpc;

pub async fn upload_status() {
    loop {
        if let Some(grpc) = get_grpc().await {
            let config = get_config().clone();
            let status = Status {
                system: get_system_status(),
            };
            let mut req = DockerRequest {
                agent_id: config.id,
                payload: serde_json::to_string(&status).expect("Failed to serialize payload"),
                ..Default::default()
            };
            req.set_action(Action::UploadStatus);
            grpc.send(req).await;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}