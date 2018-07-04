/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHardeningApplyItemResponse {
  /// Message text indicating if hardening apply operation started successfully or failed to start.
  #[serde(rename = "message")]
  message: Option<String>
}

impl CreateHardeningApplyItemResponse {
  pub fn new() -> CreateHardeningApplyItemResponse {
    CreateHardeningApplyItemResponse {
      message: None
    }
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> CreateHardeningApplyItemResponse {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

}


