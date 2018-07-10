

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCloudPoolResponse {
  /// The name of the new pool
  #[serde(rename = "id")]
  id: Option<String>
}

