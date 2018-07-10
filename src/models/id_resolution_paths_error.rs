

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdResolutionPathsError {
  #[serde(rename = "field")]
  field: Option<String>,
  #[serde(rename = "field_value")]
  field_value: Option<String>,
  #[serde(rename = "message")]
  message: Option<String>
}

