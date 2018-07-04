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
pub struct NodeDriveconfigNodeAutomaticReplacementRecognition {
  /// Enable automatic replacement recognition (ARR).
  #[serde(rename = "enabled")]
  enabled: Option<bool>
}

impl NodeDriveconfigNodeAutomaticReplacementRecognition {
  pub fn new() -> NodeDriveconfigNodeAutomaticReplacementRecognition {
    NodeDriveconfigNodeAutomaticReplacementRecognition {
      enabled: None
    }
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> NodeDriveconfigNodeAutomaticReplacementRecognition {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

}


