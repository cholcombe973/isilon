#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdResolutionPaths {
    #[serde(rename = "paths")]
    pub paths: Option<Vec <crate::models::IdResolutionPath>>,
}
