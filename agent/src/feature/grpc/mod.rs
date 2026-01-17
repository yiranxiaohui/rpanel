mod recv;
mod handle;
mod status;

use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tokio::sync::mpsc::Sender;
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use tonic::Streaming;
use tracing::{error, info};
use grpc::bridge::grpc::bridge_service_client::BridgeServiceClient;
use grpc::bridge::grpc::Frame;
use crate::config::get_config;
use crate::feature::grpc::handle::handle_message;

#[derive(Debug, Clone)]
pub struct Grpc {
    pub client: BridgeServiceClient<tonic::transport::Channel>,
    pub tx: Sender<Frame>,
}

impl Grpc {

    pub async fn create(mut client: BridgeServiceClient<tonic::transport::Channel>, id: String) -> Result<(Grpc, Streaming<Frame>), Box<dyn std::error::Error + Send + Sync>> {
        let (tx, rx) = mpsc::channel(8);
        // 1. 把 rx 包装成 Stream
        let outbound = ReceiverStream::new(rx);
        // 2. 调用 exec（把 Stream 传进去）
        let response = client.exchange(outbound).await?;
        // 3. 得到服务端返回的 stream
        let inbound = response.into_inner();
        // 发送注册请求
        let grpc = Grpc { client, tx};
        Ok((grpc, inbound))
    }

    pub async fn send(&self, frame: Frame) {
        match self.tx.send(frame).await {
            Ok(_) => {}
            Err(err) => {
                error!("send error: {}", err);
            }
        }
    }
}

pub static GRPC: RwLock<Option<Grpc>> = RwLock::const_new(None);

pub async fn init_grpc() {
    // tokio::spawn(upload_status());
    loop {
        let config = get_config().clone();
        match BridgeServiceClient::connect(config.controller).await {
            Ok(client) => {
                match Grpc::create(client, config.id).await {
                    Ok((grpc, mut inbound)) => {
                         {
                            let mut lock = GRPC.write().await;
                            *lock = Some(grpc);
                         }
                         info!("gRPC initialized and connected");
                         while let Ok(Some(frame)) = inbound.message().await {
                             handle_message(frame).await;
                         }
                         error!("Connection lost");
                    }
                    Err(e) => {
                        error!("Failed to initialize gRPC session: {}", e);
                    }
                }
            }
            Err(err) => {
                error!("gRPC connect failed: {}, reconnecting...", err);
            }
        }
        {
            let mut lock = GRPC.write().await;
            *lock = None;
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

pub async fn get_grpc() -> Option<Grpc> {
    let lock = GRPC.read().await;
    lock.clone()
}
