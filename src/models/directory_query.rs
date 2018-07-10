

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryQuery {
  #[serde(rename = "result")]
  result: Option<Vec<String>>,
  #[serde(rename = "scope")]
  scope: Option<::models::DirectoryQueryScope>
}

