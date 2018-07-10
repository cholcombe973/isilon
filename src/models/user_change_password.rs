
/// UserChangePassword : Change Password Request

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserChangePassword {
  /// Specifies user's new password
  #[serde(rename = "new_password")]
  new_password: String,
  /// User's expired password
  #[serde(rename = "old_password")]
  old_password: String
}

