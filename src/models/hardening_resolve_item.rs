/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// HardeningResolveItem : Resolve Issues found on the cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardeningResolveItem {
  /// Hardening profile.
  #[serde(rename = "profile")]
  profile: Option<String>
}

impl HardeningResolveItem {
  /// Resolve Issues found on the cluster.
  pub fn new() -> HardeningResolveItem {
    HardeningResolveItem {
      profile: None
    }
  }

  pub fn set_profile(&mut self, profile: String) {
    self.profile = Some(profile);
  }

  pub fn with_profile(mut self, profile: String) -> HardeningResolveItem {
    self.profile = Some(profile);
    self
  }

  pub fn profile(&self) -> Option<&String> {
    self.profile.as_ref()
  }

  pub fn reset_profile(&mut self) {
    self.profile = None;
  }

}


