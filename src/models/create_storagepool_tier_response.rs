

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStoragepoolTierResponse {
  /// The system ID of the new object.
  #[serde(rename = "id")]
  id: i32
}

