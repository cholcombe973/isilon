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
pub struct NfsCheck {
  /// The ID of the export.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// The message about the export.
  #[serde(rename = "message")]
  message: String
}

impl NfsCheck {
  pub fn new(message: String) -> NfsCheck {
    NfsCheck {
      id: None,
      message: message
    }
  }

  pub fn set_id(&mut self, id: i32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i32) -> NfsCheck {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i32> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = message;
  }

  pub fn with_message(mut self, message: String) -> NfsCheck {
    self.message = message;
    self
  }

  pub fn message(&self) -> &String {
    &self.message
  }


}


