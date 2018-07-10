#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HdfsRangerPluginSettingsSettings {
    /// Enable or disable the HDFS ranger plugin
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The scheme, hostname, and port of the Apache Ranger server (e.g. http://ranger.com:6080)
    #[serde(rename = "policy_manager_url")]
    pub policy_manager_url: Option<String>,
    /// The HDFS repository name that is registered with Apache Ranger server
    #[serde(rename = "repository_name")]
    pub repository_name: Option<String>,
}
