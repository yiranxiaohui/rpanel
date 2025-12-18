use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tonic::codegen::tokio_stream;
use crate::docker::grpc::{DockerReply, DockerRequest};
use crate::docker::grpc::greeter_server::Greeter;

pub mod grpc {
    tonic::include_proto!("docker");
}

#[derive(Default)]
pub struct DockerGreeter;

#[tonic::async_trait]
impl Greeter for DockerGreeter {
    type ExecStream =
    std::pin::Pin<Box<dyn tokio_stream::Stream<Item = Result<DockerReply, Status>> + Send>>;

    async fn exec(
        &self,
        request: Request<tonic::Streaming<DockerRequest>>,
    ) -> Result<Response<Self::ExecStream>, Status> {
        println!("client connected");

        // 1. 拿到客户端请求流
        let mut inbound = request.into_inner();

        // 2. 创建一个 channel，用来给客户端回数据
        let (tx, rx) = tokio::sync::mpsc::channel(16);

        // ===== 1️⃣ 处理客户端输入 =====
        let tx_input = tx.clone();
        tokio::spawn(async move {
            while let Ok(Some(req)) = inbound.message().await {
                println!("recv: {:?}", req.action);

                let reply = DockerReply {
                    action: format!("server recv action = {}", req.action),
                };

                if tx_input.send(Ok(reply)).await.is_err() {
                    break;
                }
            }

            println!("client disconnected");
        });
        // ===== 2️⃣ 服务端主动推送（心跳 / 状态）=====
        let tx_push = tx.clone();
        tokio::spawn(async move {
            let mut i = 0;
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                let msg = DockerReply {
                    action: format!("server heartbeat {}", i),
                };

                if tx_push.send(Ok(msg)).await.is_err() {
                    println!("client gone, stop push");
                    break;
                }

                i += 1;
            }
        });
        // 5. 把 rx 包装成 Stream 返回
        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }
}