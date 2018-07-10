#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareStartItem {
    /// The nodepool ID or name on which to start the upgrade.
    #[serde(rename = "node_pool")]
    pub node_pool: String,
    /// The type of upgrade to start.
    #[serde(rename = "upgrade_type")]
    pub upgrade_type: String,
}
