

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCloudJobResponse {
  /// The id of the new job
  #[serde(rename = "id")]
  id: Option<i32>
}

