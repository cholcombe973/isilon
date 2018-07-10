
/// NdmpSettingsPreferredIps : Get a list of preferred ip preferences.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsPreferredIps {
  #[serde(rename = "preferences")]
  preferences: Option<Vec<::models::NdmpSettingsPreferredIpsPreference>>,
  /// Resume string returned by previous query.
  #[serde(rename = "resume")]
  resume: Option<String>,
  /// The number of preferences.
  #[serde(rename = "total")]
  total: Option<i32>
}

