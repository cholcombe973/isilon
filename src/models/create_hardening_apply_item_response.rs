

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHardeningApplyItemResponse {
  /// Message text indicating if hardening apply operation started successfully or failed to start.
  #[serde(rename = "message")]
  message: Option<String>
}

