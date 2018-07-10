/// ClusterAddNodeItem : Add Node information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterAddNodeItem {
    /// Allow down nodes (Default false).
    #[serde(rename = "allow_down")]
    pub allow_down: Option<bool>,
    /// Serial number of this node.
    #[serde(rename = "serial_number")]
    pub serial_number: String,
    /// Bypass hardware version checks (Default false).
    #[serde(rename = "skip_hardware_version_check")]
    pub skip_hardware_version_check: Option<bool>,
}
