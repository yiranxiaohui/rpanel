use std::collections::HashMap;
use std::sync::LazyLock;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tracing::info;

static CLIENT_MAP: LazyLock<RwLock<HashMap<String, Sender<Result<Frame, Status>>>>>
    = LazyLock::new(|| RwLock::new(HashMap::new()));
//
// pub async fn send_to_agent(agent_id: &str, msg: DockerReply) -> bool {
//     let map = CLIENT_MAP.read().await;
//     if let Some(tx) = map.get(agent_id) {
//         match tx.send(Ok(msg)).await {
//             Ok(_) => return true,
//             Err(e) => {
//                 error!("Failed to send message to agent {}: {}", agent_id, e);
//             }
//         }
//     }
//     false
// }
//
use grpc::bridge::grpc::bridge_service_server::{BridgeService};
use grpc::bridge::grpc::Frame;
use crate::feature::grpc::handle::{set_agent_offline, set_agent_online};

#[derive(Default)]
#[derive(Clone)]
pub struct BridgeGreeter {}

#[tonic::async_trait]
impl BridgeService for BridgeGreeter {
    type ExchangeStream = std::pin::Pin<Box<dyn tokio_stream::Stream<Item=Result<Frame, Status>> + Send>>;

    async fn exchange(&self, request: Request<Streaming<Frame>>) -> Result<Response<Self::ExchangeStream>, Status> {
        // 1. 拿到客户端请求流
        let inbound = request.into_inner();
        // 2. 创建一个 channel，用来给客户端回数据
        let (tx, rx) = tokio::sync::mpsc::channel::<Result<Frame, Status>>(16);
        tokio::spawn(handle_message(inbound, tx));
        // 3. 把 rx 包装成 Stream 返回
        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }
}

async fn handle_message(mut inbound: Streaming<Frame>, tx: Sender<Result<Frame, Status>>) {
    let mut current_agent_id = String::new();
    while let Ok(req)= inbound.message().await {
        if let Some(frame) = req {
            current_agent_id = frame.agent_id.clone();
            info!("Handling frame {:?}", frame);
            online(current_agent_id.clone(), tx.clone()).await;
        }
    }
    offline(current_agent_id.clone()).await;
}

async fn online(agent_id: String, tx: Sender<Result<Frame, Status>>) {
    if !agent_id.is_empty() {
        info!("{} client connected", agent_id);
        CLIENT_MAP.write().await.insert(agent_id.clone(), tx.clone());
        set_agent_online(agent_id).await;
    }
}

async fn offline(agent_id: String) {
    if !agent_id.is_empty() {
        info!("{} client disconnected", agent_id);
        CLIENT_MAP.write().await.remove(&agent_id);
        set_agent_offline(agent_id).await;
    }
}


    // pub async fn handle_message(mut inbound: Streaming<DockerRequest>, tx: Sender<Result<DockerReply, Status>>) {
    //     let mut current_agent_id = String::new();
    //     while let Ok(Some(req)) = inbound.message().await {
    //         let action = req.action();
    //         match action {
    //             Action::UploadStatus => {
    //                 handle_upload_status_message(req).await;
    //             }
    //             Action::UpLine => {
    //                 let agent_id = req.agent_id;
    //                 current_agent_id = agent_id.clone();
    //                 info!("{} client connected", agent_id);
    //                 CLIENT_MAP.write().await.insert(agent_id.clone(), tx.clone());
    //                 set_agent_online(agent_id).await;
    //             }
    //             Action::RegisterAgent => {
    //                 handle_register_agent(req).await;
    //             }
    //             Action::CreateContainer => {}
    //             Action::PullImage => {
    //                 let wrapper = serde_json::json!({
    //                     "agent_id": req.agent_id,
    //                     "data": req.payload
    //                 });
    //                 send_event("pull_progress", wrapper.to_string());
    //             }
    //             Action::ListContainers => {
    //                 use crate::feature::grpc::handle::handle_docker_info_message;
    //                 handle_docker_info_message(req.agent_id, 1, req.payload).await;
    //             }
    //             Action::ListImages => {
    //                 use crate::feature::grpc::handle::handle_docker_info_message;
    //                 handle_docker_info_message(req.agent_id, 2, req.payload).await;
    //             }
    //             _ => {}
    //         }
    //     }
    //
    //     if !current_agent_id.is_empty() {
    //         info!("{} client disconnected", current_agent_id);
    //         CLIENT_MAP.write().await.remove(&current_agent_id);
    //         set_agent_offline(current_agent_id).await;
    //     }
    // }
