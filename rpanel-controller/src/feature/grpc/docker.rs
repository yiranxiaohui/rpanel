use std::collections::HashMap;
use std::sync::LazyLock;
use tokio::sync::mpsc::{Sender};
use tokio::sync::RwLock;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tracing::info;
use rpanel_grpc::docker::grpc::{Action, DockerReply, DockerRequest};
use rpanel_grpc::docker::grpc::greeter_server::Greeter;
use crate::feature::grpc::handle::{handle_register_agent, handle_upload_status_message, set_agent_offline, set_agent_online};

static CLIENT_MAP: LazyLock<RwLock<HashMap<String, Sender<Result<DockerReply, Status>>>>>
    = LazyLock::new(|| RwLock::new(HashMap::new()));

#[derive(Default)]
pub struct DockerGreeter {}

#[tonic::async_trait]
impl Greeter for DockerGreeter {
    type ExecStream =
    std::pin::Pin<Box<dyn tokio_stream::Stream<Item = Result<DockerReply, Status>> + Send>>;

    async fn exec(
        &self,
        request: Request<tonic::Streaming<DockerRequest>>,
    ) -> Result<Response<Self::ExecStream>, Status> {
        // 1. 拿到客户端请求流
        let inbound = request.into_inner();
        // 2. 创建一个 channel，用来给客户端回数据
        let (tx, rx) = tokio::sync::mpsc::channel::<Result<DockerReply, Status>>(16);
        tokio::spawn(handle_message(inbound, tx));
        // 3. 把 rx 包装成 Stream 返回
        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }
}

pub async fn handle_message(mut inbound: Streaming<DockerRequest>, tx: Sender<Result<DockerReply, Status>>) {
    let mut current_agent_id = String::new();
    while let Ok(Some(req)) = inbound.message().await {
        let action = req.action();
        match action {
            Action::UploadStatus => {
                handle_upload_status_message(req).await;
            }
            Action::UpLine => {
                let agent_id = req.agent_id;
                current_agent_id = agent_id.clone();
                info!("{} client connected", agent_id);
                CLIENT_MAP.write().await.insert(agent_id.clone(), tx.clone());
                set_agent_online(agent_id).await;
            }
            Action::RegisterAgent => {
                handle_register_agent(req).await;
            }
            Action::CreateContainer => {}
            Action::PullImage => {}
        }
    }

    if !current_agent_id.is_empty() {
        info!("{} client disconnected", current_agent_id);
        CLIENT_MAP.write().await.remove(&current_agent_id);
        set_agent_offline(current_agent_id).await;
    }
}
