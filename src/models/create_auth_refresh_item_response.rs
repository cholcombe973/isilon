

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAuthRefreshItemResponse {
  /// Unique ID of the log filter.
  #[serde(rename = "id")]
  id: Option<i32>
}

