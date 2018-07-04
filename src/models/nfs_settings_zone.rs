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
pub struct NfsSettingsZone {
  /// Specifies the per-zone NFS configuration settings.
  #[serde(rename = "settings")]
  settings: Option<::models::NfsSettingsZoneSettings>
}

impl NfsSettingsZone {
  pub fn new() -> NfsSettingsZone {
    NfsSettingsZone {
      settings: None
    }
  }

  pub fn set_settings(&mut self, settings: ::models::NfsSettingsZoneSettings) {
    self.settings = Some(settings);
  }

  pub fn with_settings(mut self, settings: ::models::NfsSettingsZoneSettings) -> NfsSettingsZone {
    self.settings = Some(settings);
    self
  }

  pub fn settings(&self) -> Option<&::models::NfsSettingsZoneSettings> {
    self.settings.as_ref()
  }

  pub fn reset_settings(&mut self) {
    self.settings = None;
  }

}


