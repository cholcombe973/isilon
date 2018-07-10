
/// SwiftAccount : This is an account for the Swift protocol.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SwiftAccount {
  /// Unique id of swift account
  #[serde(rename = "id")]
  id: Option<String>,
  /// Name of Swift account
  #[serde(rename = "name")]
  name: String,
  /// Group with filesystem ownership of this account
  #[serde(rename = "swiftgroup")]
  swiftgroup: Option<String>,
  /// User with filesystem ownership of this account
  #[serde(rename = "swiftuser")]
  swiftuser: Option<String>,
  /// Users who are allowed to access Swift account
  #[serde(rename = "users")]
  users: Option<Vec<String>>,
  /// Name of access zone for account
  #[serde(rename = "zone")]
  zone: Option<String>
}

