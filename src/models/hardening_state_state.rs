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
pub struct HardeningStateState {
  /// Full path name of issues file, or null if no issues file is configured. This file contains all issues found when the cluster configuration is checked against expected configuration.
  #[serde(rename = "issues_file")]
  issues_file: Option<String>,
  /// This contains more information and details about the operation state.
  #[serde(rename = "message")]
  message: Option<String>,
  /// The state of the hardening operation. In case there is no operation currently going on, this will display the last state of operation.
  #[serde(rename = "state")]
  state: Option<String>
}

impl HardeningStateState {
  pub fn new() -> HardeningStateState {
    HardeningStateState {
      issues_file: None,
      message: None,
      state: None
    }
  }

  pub fn set_issues_file(&mut self, issues_file: String) {
    self.issues_file = Some(issues_file);
  }

  pub fn with_issues_file(mut self, issues_file: String) -> HardeningStateState {
    self.issues_file = Some(issues_file);
    self
  }

  pub fn issues_file(&self) -> Option<&String> {
    self.issues_file.as_ref()
  }

  pub fn reset_issues_file(&mut self) {
    self.issues_file = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> HardeningStateState {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_state(&mut self, state: String) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: String) -> HardeningStateState {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&String> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

}


