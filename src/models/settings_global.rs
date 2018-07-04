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
pub struct SettingsGlobal {
  /// Specifies the properties for global authentication settings.
  #[serde(rename = "global_settings")]
  global_settings: Option<::models::SettingsGlobalGlobalSettings>
}

impl SettingsGlobal {
  pub fn new() -> SettingsGlobal {
    SettingsGlobal {
      global_settings: None
    }
  }

  pub fn set_global_settings(&mut self, global_settings: ::models::SettingsGlobalGlobalSettings) {
    self.global_settings = Some(global_settings);
  }

  pub fn with_global_settings(mut self, global_settings: ::models::SettingsGlobalGlobalSettings) -> SettingsGlobal {
    self.global_settings = Some(global_settings);
    self
  }

  pub fn global_settings(&self) -> Option<&::models::SettingsGlobalGlobalSettings> {
    self.global_settings.as_ref()
  }

  pub fn reset_global_settings(&mut self) {
    self.global_settings = None;
  }

}


