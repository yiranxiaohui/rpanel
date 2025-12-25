use rpanel_common::docker::{
    PullImageRequest, RunContainerRequest, 
    ContainerActionRequest, RemoveContainerRequest, RemoveImageRequest,
    ContainerInfo, ImageInfo, Port
};
use rpanel_grpc::docker::grpc::{Action, DockerReply, DockerRequest};
use tracing::{error, info};
use crate::feature::docker::image::{pull_image, get_image_list, remove_image};
use crate::feature::docker::container::{
    get_container_list, start_container, stop_container, remove_container, create_container
};
use crate::feature::grpc::get_grpc;
use crate::config::get_config;

pub async fn handle_message(reply: DockerReply) {
    let action = reply.action();
    let payload = reply.payload.clone();
    let config = get_config();

    match action {
        Action::UploadStatus => {}
        Action::PullImage => {
            if let Ok(req) = serde_json::from_str::<PullImageRequest>(&payload) {
                tokio::spawn(async move {
                    if let Err(e) = pull_image(req.image).await {
                        error!("Failed to pull image: {}", e);
                    }
                });
            }
        }
        Action::ListContainers => {
            tokio::spawn(async move {
                match get_container_list().await {
                    Ok(containers) => {
                         // Convert to rpanel-common struct
                         let infos: Vec<ContainerInfo> = containers.into_iter().map(|c| {
                             ContainerInfo {
                                 id: c.id.unwrap_or_default(),
                                 names: c.names.unwrap_or_default(),
                                 image: c.image.unwrap_or_default(),
                                 image_id: c.image_id.unwrap_or_default(),
                                 command: c.command.unwrap_or_default(),
                                 created: c.created.unwrap_or_default(),
                                 state: c.state.map(|s| s.to_string()).unwrap_or_default(),
                                 status: c.status.unwrap_or_default(),
                                 ports: c.ports.unwrap_or_default().into_iter().map(|p| Port {
                                     ip: p.ip,
                                     private_port: p.private_port,
                                     public_port: p.public_port,
                                     type_: p.typ.map(|t| t.to_string()).unwrap_or_default(),
                                 }).collect(),
                             }
                         }).collect();

                         if let Some(grpc) = get_grpc().await {
                            let mut request = DockerRequest {
                                agent_id: config.id.clone(),
                                payload: serde_json::to_string(&infos).unwrap_or_default(),
                                ..Default::default()
                            };
                            request.set_action(Action::ListContainers);
                            grpc.send(request).await;
                        }
                    }
                    Err(e) => error!("Failed to list containers: {}", e),
                }
            });
        }
        Action::ListImages => {
             tokio::spawn(async move {
                match get_image_list().await {
                    Ok(images) => {
                         let infos: Vec<ImageInfo> = images.into_iter().map(|i| {
                             ImageInfo {
                                 id: i.id,
                                 repo_tags: i.repo_tags,
                                 created: i.created,
                                 size: i.size,
                             }
                         }).collect();

                         if let Some(grpc) = get_grpc().await {
                            let mut request = DockerRequest {
                                agent_id: config.id.clone(),
                                payload: serde_json::to_string(&infos).unwrap_or_default(),
                                ..Default::default()
                            };
                            request.set_action(Action::ListImages);
                            grpc.send(request).await;
                        }
                    }
                    Err(e) => error!("Failed to list images: {}", e),
                }
            });
        }
        Action::StartContainer => {
            if let Ok(req) = serde_json::from_str::<ContainerActionRequest>(&payload) {
                tokio::spawn(async move {
                    if let Err(e) = start_container(req.container_id).await {
                        error!("Failed to start container: {}", e);
                    } else {
                        info!("Started container");
                        // Trigger list update
                        // ...
                    }
                });
            }
        }
        Action::StopContainer => {
             if let Ok(req) = serde_json::from_str::<ContainerActionRequest>(&payload) {
                tokio::spawn(async move {
                    if let Err(e) = stop_container(req.container_id).await {
                        error!("Failed to stop container: {}", e);
                    }
                });
            }
        }
        Action::RemoveContainer => {
             if let Ok(req) = serde_json::from_str::<RemoveContainerRequest>(&payload) {
                tokio::spawn(async move {
                    if let Err(e) = remove_container(req.container_id).await {
                        error!("Failed to remove container: {}", e);
                    }
                });
            }
        }
        Action::RemoveImage => {
             if let Ok(req) = serde_json::from_str::<RemoveImageRequest>(&payload) {
                tokio::spawn(async move {
                    if let Err(e) = remove_image(req.image_id, req.force).await {
                        error!("Failed to remove image: {}", e);
                    }
                });
            }
        }
        Action::RunContainer => {
             if let Ok(req) = serde_json::from_str::<RunContainerRequest>(&payload) {
                tokio::spawn(async move {
                    match create_container(req.image, req.name.unwrap_or_default(), req.command, req.ports).await {
                        Ok(resp) => {
                             if let Err(e) = start_container(resp.id).await {
                                 error!("Failed to start new container: {}", e);
                             }
                        }
                        Err(e) => error!("Failed to create container: {}", e),
                    }
                });
            }
        }
        _ => {}
    }
}