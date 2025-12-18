use bollard::errors::Error;
use bollard::models::{ContainerCreateBody, ContainerCreateResponse, ContainerSummary};
use bollard::query_parameters::{
    CreateContainerOptions,
    StartContainerOptions,
    StopContainerOptions,
    RemoveContainerOptions,
    ListContainersOptions,
};
use tracing::{error, info};

use crate::feature::docker::get_docker;


/// 创建容器
pub async fn create_container(
    image: String,
    name: String,
) -> Result<ContainerCreateResponse, Error> {
    let docker = get_docker();

    let options = Some(CreateContainerOptions {
        name: Some(name),
        ..Default::default()
    });

    let config = ContainerCreateBody {
        image: Some(image),
        ..Default::default()
    };

    match docker.create_container(options, config).await {
        Ok(resp) => {
            info!("container created: {:?}", resp.id);
            Ok(resp)
        }
        Err(err) => {
            error!("Failed to create container: {}", err);
            Err(err)
        }
    }
}

/// 启动容器
pub async fn start_container(id: String) -> Result<(), Error> {
    let docker = get_docker();

    docker
        .start_container(&id, None::<StartContainerOptions>)
        .await
        .map_err(|err| {
            error!("Failed to start container {}: {}", id, err);
            err
        })
}

/// 停止容器
pub async fn stop_container(id: String) -> Result<(), Error> {
    let docker = get_docker();

    docker
        .stop_container(&id, None::<StopContainerOptions>)
        .await
        .map_err(|err| {
            error!("Failed to stop container {}: {}", id, err);
            err
        })
}

/// 删除容器
pub async fn remove_container(id: String) -> Result<(), Error> {
    let docker = get_docker();

    let options = Some(RemoveContainerOptions {
        force: true,
        ..Default::default()
    });

    docker
        .remove_container(&id, options)
        .await
        .map_err(|err| {
            error!("Failed to remove container {}: {}", id, err);
            err
        })
}

/// 获取容器列表
pub async fn get_container_list() -> Result<Vec<ContainerSummary>, Error> {
    let docker = get_docker();

    let options = Some(ListContainersOptions {
        all: true,
        ..Default::default()
    });

    docker.list_containers(options).await.map_err(|err| {
        error!("Failed to list containers: {}", err);
        err
    })
}
