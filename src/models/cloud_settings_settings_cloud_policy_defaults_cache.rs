#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudSettingsSettingsCloudPolicyDefaultsCache {
    /// Specifies cache expiration.
    #[serde(rename = "expiration")]
    pub expiration: Option<i32>,
    /// Specifies cache read ahead type.
    #[serde(rename = "read_ahead")]
    pub read_ahead: Option<String>,
    /// Specifies cache type.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
