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
pub struct CopyErrorsCopyErrors {
  #[serde(rename = "error_src")]
  error_src: Option<String>,
  #[serde(rename = "message")]
  message: Option<String>,
  #[serde(rename = "source")]
  source: Option<String>,
  #[serde(rename = "target")]
  target: Option<String>
}

impl CopyErrorsCopyErrors {
  pub fn new() -> CopyErrorsCopyErrors {
    CopyErrorsCopyErrors {
      error_src: None,
      message: None,
      source: None,
      target: None
    }
  }

  pub fn set_error_src(&mut self, error_src: String) {
    self.error_src = Some(error_src);
  }

  pub fn with_error_src(mut self, error_src: String) -> CopyErrorsCopyErrors {
    self.error_src = Some(error_src);
    self
  }

  pub fn error_src(&self) -> Option<&String> {
    self.error_src.as_ref()
  }

  pub fn reset_error_src(&mut self) {
    self.error_src = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> CopyErrorsCopyErrors {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_source(&mut self, source: String) {
    self.source = Some(source);
  }

  pub fn with_source(mut self, source: String) -> CopyErrorsCopyErrors {
    self.source = Some(source);
    self
  }

  pub fn source(&self) -> Option<&String> {
    self.source.as_ref()
  }

  pub fn reset_source(&mut self) {
    self.source = None;
  }

  pub fn set_target(&mut self, target: String) {
    self.target = Some(target);
  }

  pub fn with_target(mut self, target: String) -> CopyErrorsCopyErrors {
    self.target = Some(target);
    self
  }

  pub fn target(&self) -> Option<&String> {
    self.target.as_ref()
  }

  pub fn reset_target(&mut self) {
    self.target = None;
  }

}


