

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHardeningResolveItemResponse {
  /// Message text indicating if operation to resolve issues started successfully or failed to start.
  #[serde(rename = "message")]
  message: Option<String>
}

