use common::status::{get_system_status, Status};
use grpc::bridge::grpc::{CallRequest, Frame};
use crate::config::get_config;
use crate::feature::grpc::get_grpc;

pub async fn upload_status() {
    loop {
        if let Some(grpc) = get_grpc().await {
            let config = get_config().clone();
            let status = Status {
                system: get_system_status(),
            };
            let mut frame = Frame {
                agent_id: config.id,
                ..Default::default()
            };
            frame.body = Some(grpc::bridge::grpc::frame::Body::Call(CallRequest{
                method: "upload_status".to_string(),
                payload: vec![],
            }));
            grpc.send(frame).await;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}