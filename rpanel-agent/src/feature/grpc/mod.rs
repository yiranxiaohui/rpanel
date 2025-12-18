mod recv;
mod handle;

use tokio::sync::{mpsc, OnceCell};
use tokio::sync::mpsc::{Sender};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use tracing::{error};
use rpanel_grpc::docker::grpc::{DockerRequest};
use rpanel_grpc::docker::grpc::greeter_client::GreeterClient;
use crate::config::get_config;
use crate::feature::grpc::handle::handle_message;

pub struct Grpc {
    pub client: GreeterClient<tonic::transport::Channel>,
    pub tx: Sender<DockerRequest>,
}

impl Grpc {

    pub async fn new(mut client: GreeterClient<tonic::transport::Channel>) -> Grpc {
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
                handle_message(reply);
            }
        });

        Grpc { client, tx}
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

pub static GRPC: OnceCell<Grpc> = OnceCell::const_new();

pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config().clone();
    let client = GreeterClient::connect(config.url).await?;
    GRPC.get_or_init(|| {
        Grpc::new(client)
    }).await;
    Ok(())
}

pub async fn get_grpc() -> Option<&'static Grpc> {
    GRPC.get()
}