#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodeStateSmartfail {
    /// This node is smartfailed (soft_devs).
    #[serde(rename = "smartfailed")]
    pub smartfailed: Option<bool>,
}
