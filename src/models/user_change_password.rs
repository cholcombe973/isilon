/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

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

impl UserChangePassword {
  /// Change Password Request
  pub fn new(new_password: String, old_password: String) -> UserChangePassword {
    UserChangePassword {
      new_password: new_password,
      old_password: old_password
    }
  }

  pub fn set_new_password(&mut self, new_password: String) {
    self.new_password = new_password;
  }

  pub fn with_new_password(mut self, new_password: String) -> UserChangePassword {
    self.new_password = new_password;
    self
  }

  pub fn new_password(&self) -> &String {
    &self.new_password
  }


  pub fn set_old_password(&mut self, old_password: String) {
    self.old_password = old_password;
  }

  pub fn with_old_password(mut self, old_password: String) -> UserChangePassword {
    self.old_password = old_password;
    self
  }

  pub fn old_password(&self) -> &String {
    &self.old_password
  }


}


