/// NdmpSettingsPreferredIps : Get a list of preferred ip preferences.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsPreferredIps {
    #[serde(rename = "preferences")]
    pub preferences: Option<Vec<::models::NdmpSettingsPreferredIpsPreference>>,
    /// Resume string returned by previous query.
    #[serde(rename = "resume")]
    pub resume: Option<String>,
    /// The number of preferences.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
