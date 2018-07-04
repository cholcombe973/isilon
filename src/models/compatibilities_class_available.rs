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
pub struct CompatibilitiesClassAvailable {
  #[serde(rename = "available")]
  available: Option<Vec<::models::CompatibilitiesClassAvailableAvailableItem>>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

impl CompatibilitiesClassAvailable {
  pub fn new() -> CompatibilitiesClassAvailable {
    CompatibilitiesClassAvailable {
      available: None,
      total: None
    }
  }

  pub fn set_available(&mut self, available: Vec<::models::CompatibilitiesClassAvailableAvailableItem>) {
    self.available = Some(available);
  }

  pub fn with_available(mut self, available: Vec<::models::CompatibilitiesClassAvailableAvailableItem>) -> CompatibilitiesClassAvailable {
    self.available = Some(available);
    self
  }

  pub fn available(&self) -> Option<&Vec<::models::CompatibilitiesClassAvailableAvailableItem>> {
    self.available.as_ref()
  }

  pub fn reset_available(&mut self) {
    self.available = None;
  }

  pub fn set_total(&mut self, total: i32) {
    self.total = Some(total);
  }

  pub fn with_total(mut self, total: i32) -> CompatibilitiesClassAvailable {
    self.total = Some(total);
    self
  }

  pub fn total(&self) -> Option<&i32> {
    self.total.as_ref()
  }

  pub fn reset_total(&mut self) {
    self.total = None;
  }

}


