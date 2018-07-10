

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersFile {
  #[serde(rename = "file")]
  file: Option<Vec<::models::ProvidersFileFileItem>>
}

