

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCloudAccountResponse {
  /// The name of the new account
  #[serde(rename = "id")]
  id: Option<String>
}

