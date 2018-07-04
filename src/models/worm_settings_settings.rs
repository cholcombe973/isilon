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
pub struct WormSettingsSettings {
  /// Specifies the current time of the SmartLock compliance clock in Unix Epoch seconds. If the compliance clock is not set, this value is null. A PUT request will set the compliance clock date to the current system time. The cluster must be in compliance mode to set the compliance clock.
  #[serde(rename = "cdate")]
  cdate: Option<i32>
}

impl WormSettingsSettings {
  pub fn new() -> WormSettingsSettings {
    WormSettingsSettings {
      cdate: None
    }
  }

  pub fn set_cdate(&mut self, cdate: i32) {
    self.cdate = Some(cdate);
  }

  pub fn with_cdate(mut self, cdate: i32) -> WormSettingsSettings {
    self.cdate = Some(cdate);
    self
  }

  pub fn cdate(&self) -> Option<&i32> {
    self.cdate.as_ref()
  }

  pub fn reset_cdate(&mut self) {
    self.cdate = None;
  }

}


