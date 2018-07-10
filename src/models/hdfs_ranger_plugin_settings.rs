#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HdfsRangerPluginSettings {
    /// Settings for the HDFS ranger plugin
    #[serde(rename = "settings")]
    pub settings: Option<::models::HdfsRangerPluginSettingsSettings>,
}
