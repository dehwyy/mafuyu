use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PublishImageRequest {
    pub filename: String,
    pub base64_image: String,
    pub image_ext: String,
}

pub mod subject {
    pub const PUBLISH_IMAGE: &str = "cdn.do.publish_image";
}
