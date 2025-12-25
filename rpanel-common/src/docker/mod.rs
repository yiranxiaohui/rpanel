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
