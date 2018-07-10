

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthIdNtokenPrivilegeItem {
  /// Specifies the ID of the privilege.
  #[serde(rename = "id")]
  id: String,
  /// Specifies the name of the privilege.
  #[serde(rename = "name")]
  name: Option<String>,
  /// True, if the privilege is read-only.
  #[serde(rename = "read_only")]
  read_only: Option<bool>
}

