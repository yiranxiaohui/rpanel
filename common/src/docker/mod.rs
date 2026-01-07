use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PullImageRequest {
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullImageProgress {
    pub status: Option<String>,
    pub progress: Option<String>,
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunContainerRequest {
    pub image: String,
    pub command: Option<String>, // "command args"
    pub name: Option<String>,
    pub ports: Option<Vec<(String, String)>>, // host:container
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerActionRequest {
    pub container_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveContainerRequest {
    pub container_id: String,
    pub force: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveImageRequest {
    pub image_id: String,
    pub force: bool,
}

// Data structures for reporting back to controller

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    pub image_id: String,
    pub command: String,
    pub created: i64,
    pub state: String,
    pub status: String,
    pub ports: Vec<Port>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Port {
    pub ip: Option<String>,
    pub private_port: u16,
    pub public_port: Option<u16>,
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageInfo {
    pub id: String,
    pub repo_tags: Vec<String>,
    pub created: i64,
    pub size: i64,
}