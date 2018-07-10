/// NodeStateSmartfailExtended : Node smartfail state.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStateSmartfailExtended {
    /// This node is dead (dead_devs).
    #[serde(rename = "dead")]
    pub dead: Option<bool>,
    /// This node is down (down_devs).
    #[serde(rename = "down")]
    pub down: Option<bool>,
    /// This node is in the cluster (all_devs).
    #[serde(rename = "in_cluster")]
    pub in_cluster: Option<bool>,
    /// This node is readonly (ro_devs).
    #[serde(rename = "readonly")]
    pub readonly: Option<bool>,
    /// This node is shutdown readonly (down_devs).
    #[serde(rename = "shutdown_readonly")]
    pub shutdown_readonly: Option<bool>,
    /// This node is smartfailed (soft_devs).
    #[serde(rename = "smartfailed")]
    pub smartfailed: Option<bool>,
}
