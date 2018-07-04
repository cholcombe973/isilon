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
pub struct CopyErrors {
  #[serde(rename = "copy_errors")]
  copy_errors: Option<Vec<::models::CopyErrorsCopyErrors>>
}

impl CopyErrors {
  pub fn new() -> CopyErrors {
    CopyErrors {
      copy_errors: None
    }
  }

  pub fn set_copy_errors(&mut self, copy_errors: Vec<::models::CopyErrorsCopyErrors>) {
    self.copy_errors = Some(copy_errors);
  }

  pub fn with_copy_errors(mut self, copy_errors: Vec<::models::CopyErrorsCopyErrors>) -> CopyErrors {
    self.copy_errors = Some(copy_errors);
    self
  }

  pub fn copy_errors(&self) -> Option<&Vec<::models::CopyErrorsCopyErrors>> {
    self.copy_errors.as_ref()
  }

  pub fn reset_copy_errors(&mut self) {
    self.copy_errors = None;
  }

}


