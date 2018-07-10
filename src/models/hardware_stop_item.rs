#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareStopItem {
    /// The nodepool ID or name on which to stop the upgrade.
    #[serde(rename = "node_pool")]
    pub node_pool: String,
    /// Argument to indicate whether the nodepool should split into upgraded and non-upgraded pools or not. Default is false.
    #[serde(rename = "split")]
    pub split: Option<bool>,
}
