#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HdfsSettings {
    /// This is schema that contains HDFS protocol properties.
    #[serde(rename = "settings")]
    pub settings: Option<::models::HdfsSettingsSettings>,
}
