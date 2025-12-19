use rpanel_grpc::docker::grpc::DockerReply;
use tokio::sync::mpsc::Sender;

pub struct Grpc {
    pub tx: Sender<DockerReply>
}

pub mod docker;
mod handle;