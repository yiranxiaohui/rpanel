mod recv;
mod handle;
mod status;

use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use tonic::transport::{Channel, Error};
use tracing::{error, info};
use rpanel_common::agent::AgentRegisterRequest;
use rpanel_grpc::docker::grpc::{Action, DockerRequest};
use rpanel_grpc::docker::grpc::greeter_client::GreeterClient;
use crate::config::get_config;
use crate::feature::grpc::handle::handle_message;
use crate::feature::grpc::status::upload_status;

#[derive(Debug, Clone)]
pub struct Grpc {
    pub client: GreeterClient<tonic::transport::Channel>,
    pub tx: Sender<DockerRequest>,
}

impl Grpc {

    pub async fn new(mut client: GreeterClient<tonic::transport::Channel>, id: String) -> Grpc {
        let (tx, rx) = mpsc::channel(8);
        // 1. 把 rx 包装成 Stream
        let outbound = ReceiverStream::new(rx);

        // 2. 调用 exec（把 Stream 传进去）
        let response = client.exec(outbound).await.expect("failed to execute grpc server");

        // 3. 得到服务端返回的 stream
        let mut inbound = response.into_inner();

        // 4. 接收服务端返回
        tokio::spawn(async move {
            while let Some(reply) = inbound.message().await.expect("recv connection error") {
                handle_message(reply).await;
            }
        });

        // 发送注册请求
        let register_info = AgentRegisterRequest::new("RPanel Agent".to_string());
        let mut register_req = DockerRequest {
            agent_id: id.clone(),
            payload: serde_json::to_string(&register_info).unwrap(),
            ..Default::default()
        };
        register_req.set_action(Action::RegisterAgent);
        tx.send(register_req).await.unwrap();

        let mut req = DockerRequest {
            agent_id: id.clone(),
            payload: "".to_string(),
            ..Default::default()
        };
        req.set_action(Action::UpLine);
        tx.send(req).await.unwrap();
        tokio::spawn(upload_status());
        let grpc = Grpc { client, tx};
        grpc
    }

    pub async fn send(&self, req: DockerRequest) {
        match self.tx.send(req).await {
            Ok(_) => {}
            Err(err) => {
                error!("send error: {}", err);
            }
        }
    }
}

pub static GRPC: RwLock<Option<Grpc>> = RwLock::const_new(None);

pub async fn init_grpc() {
    loop {
        let config = get_config().clone();
        match GreeterClient::connect(config.controller).await {
            Ok(client) => {
                let grpc = Grpc::new(client, config.id).await;
                let mut lock = GRPC.write().await;
                *lock = Some(grpc);
                info!("gRPC initialized");
                break;
            }
            Err(err) => {
                error!("gRPC connect failed: {}, reconnecting...", err);
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        }
    }
}

pub async fn get_grpc() -> Option<Grpc> {
    let lock = GRPC.read().await;
    lock.clone()
}
