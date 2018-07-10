#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodeExtended {
    /// An object containing a node's drive subsystem XML configuration file.
    #[serde(rename = "drive_d_config")]
    pub drive_d_config: Option<::models::ClusterNodeDriveDConfig>,
    /// List of the drives in this node.
    #[serde(rename = "drives")]
    pub drives: Option<Vec<::models::NodeDrivesNodeDrive>>,
    /// Node hardware identifying information (static).
    #[serde(rename = "hardware")]
    pub hardware: Option<::models::ClusterNodeHardware>,
    /// Node ID (Device Number) of this node.
    #[serde(rename = "id")]
    pub id: Option<i32>,
    /// Logical Node Number (LNN) of this node.
    #[serde(rename = "lnn")]
    pub lnn: Option<i32>,
    /// Node partition information.
    #[serde(rename = "partitions")]
    pub partitions: Option<::models::ClusterNodePartitions>,
    /// Node sensor information (hardware reported).
    #[serde(rename = "sensors")]
    pub sensors: Option<::models::ClusterNodeSensors>,
    /// List of the sleds in this node.
    #[serde(rename = "sleds")]
    pub sleds: Option<Vec<::models::NodeSledsNodeSled>>,
    /// Node state information (reported and modifiable).
    #[serde(rename = "state")]
    pub state: Option<::models::ClusterNodeStateExtended>,
    /// Node status information (hardware reported).
    #[serde(rename = "status")]
    pub status: Option<::models::ClusterNodeStatus>,
}
