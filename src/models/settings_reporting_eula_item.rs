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
pub struct SettingsReportingEulaItem {
  /// Indicates whether the telemetry collection warning has been acknowledged
  #[serde(rename = "accepted")]
  accepted: Option<bool>,
  /// The body of the telemetry collection warning
  #[serde(rename = "body")]
  body: Option<String>
}

impl SettingsReportingEulaItem {
  pub fn new() -> SettingsReportingEulaItem {
    SettingsReportingEulaItem {
      accepted: None,
      body: None
    }
  }

  pub fn set_accepted(&mut self, accepted: bool) {
    self.accepted = Some(accepted);
  }

  pub fn with_accepted(mut self, accepted: bool) -> SettingsReportingEulaItem {
    self.accepted = Some(accepted);
    self
  }

  pub fn accepted(&self) -> Option<&bool> {
    self.accepted.as_ref()
  }

  pub fn reset_accepted(&mut self) {
    self.accepted = None;
  }

  pub fn set_body(&mut self, body: String) {
    self.body = Some(body);
  }

  pub fn with_body(mut self, body: String) -> SettingsReportingEulaItem {
    self.body = Some(body);
    self
  }

  pub fn body(&self) -> Option<&String> {
    self.body.as_ref()
  }

  pub fn reset_body(&mut self) {
    self.body = None;
  }

}


