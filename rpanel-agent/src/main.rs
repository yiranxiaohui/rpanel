use tokio::sync::mpsc;
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use rpanel_grpc::docker::grpc::DockerRequest;
use rpanel_grpc::docker::grpc::greeter_client::GreeterClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    // 1. 创建 channel
    let (tx, rx) = mpsc::channel(8);

    // 2. 把 rx 包装成 Stream
    let outbound = ReceiverStream::new(rx);

    // 3. 调用 exec（把 Stream 传进去）
    let response = client.exec(outbound).await?;

    // 4. 得到服务端返回的 stream
    let mut inbound = response.into_inner();
    // 5. 发送请求（模拟 stdin / 指令）
    tx.send(DockerRequest {
        action: "UploadStatus".into(),
    }).await?;
    // 6. 接收服务端返回
    while let Some(reply) = inbound.message().await? {
        println!("REPLY = {:?}", reply);
    }

    Ok(())
}