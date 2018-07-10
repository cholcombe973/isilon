

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdResolutionPaths {
  #[serde(rename = "paths")]
  paths: Option<Vec<::models::IdResolutionPath>>
}

