use rpanel_grpc::docker::grpc::{Action, DockerReply};

pub async fn handle_message(reply: DockerReply) {
    let action = reply.action();
    match action {
        Action::UploadStatus => {

        }
        Action::CreateContainer => {

        },
        Action::PullImage => {
            // let payload = reply.payload.unwrap();
            // match payload {
            //     Payload::Container(_) => {}
            //     Payload::Image(image) => {
            //         pull_image(image.name).await.expect("TODO: panic message");
            //     }
            // }
        }
        _ => {}
    }
}