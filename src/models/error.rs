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
pub struct Error {
  #[serde(rename = "code")]
  code: i32,
  #[serde(rename = "message")]
  message: String
}

impl Error {
  pub fn new(code: i32, message: String) -> Error {
    Error {
      code: code,
      message: message
    }
  }

  pub fn set_code(&mut self, code: i32) {
    self.code = code;
  }

  pub fn with_code(mut self, code: i32) -> Error {
    self.code = code;
    self
  }

  pub fn code(&self) -> &i32 {
    &self.code
  }


  pub fn set_message(&mut self, message: String) {
    self.message = message;
  }

  pub fn with_message(mut self, message: String) -> Error {
    self.message = message;
    self
  }

  pub fn message(&self) -> &String {
    &self.message
  }


}


