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
pub struct SmbSettingsGlobalSettingsAuditGlobalSaclItem {
  /// Determines if audit is performed on success or failure.
  #[serde(rename = "flags")]
  flags: String,
  /// Specifies the array of filesystem rights that are governed.
  #[serde(rename = "permission")]
  permission: Vec<String>
}

impl SmbSettingsGlobalSettingsAuditGlobalSaclItem {
  pub fn new(flags: String, permission: Vec<String>) -> SmbSettingsGlobalSettingsAuditGlobalSaclItem {
    SmbSettingsGlobalSettingsAuditGlobalSaclItem {
      flags: flags,
      permission: permission
    }
  }

  pub fn set_flags(&mut self, flags: String) {
    self.flags = flags;
  }

  pub fn with_flags(mut self, flags: String) -> SmbSettingsGlobalSettingsAuditGlobalSaclItem {
    self.flags = flags;
    self
  }

  pub fn flags(&self) -> &String {
    &self.flags
  }


  pub fn set_permission(&mut self, permission: Vec<String>) {
    self.permission = permission;
  }

  pub fn with_permission(mut self, permission: Vec<String>) -> SmbSettingsGlobalSettingsAuditGlobalSaclItem {
    self.permission = permission;
    self
  }

  pub fn permission(&self) -> &Vec<String> {
    &self.permission
  }


}


