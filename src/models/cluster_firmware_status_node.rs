#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterFirmwareStatusNode {
    /// List of the firmware status for hardware components on the node.
    #[serde(rename = "devices")]
    pub devices: Option<Vec <crate::models::ClusterFirmwareStatusNodeDevice>>,
    /// The lnn of the node.
    #[serde(rename = "lnn")]
    pub lnn: Option<i32>,
    /// Node is unavailable.
    #[serde(rename = "node_unavailable")]
    pub node_unavailable: Option<bool>,
    /// List of the firmware binary information for the installed firmware package.
    #[serde(rename = "package")]
    pub package: Option<Vec <crate::models::ClusterFirmwareStatusNodePackageItem>>,
}
