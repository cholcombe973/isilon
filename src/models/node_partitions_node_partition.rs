#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodePartitionsNodePartition {
    /// The block size used for the reported partition information.
    #[serde(rename = "block_size")]
    pub block_size: Option<i32>,
    /// Total blocks on this file system partition.
    #[serde(rename = "capacity")]
    pub capacity: Option<i32>,
    /// Comma separated list of devices used for this file system partition.
    #[serde(rename = "component_devices")]
    pub component_devices: Option<String>,
    /// Directory on which this partition is mounted.
    #[serde(rename = "mount_point")]
    pub mount_point: Option<String>,
    /// Used blocks on this file system partition, expressed as a percentage.
    #[serde(rename = "percent_used")]
    pub percent_used: Option<String>,
    /// System partition details as provided by statfs(2).
    #[serde(rename = "statfs")]
    pub statfs: Option <crate::models::NodePartitionsNodePartitionStatfs>,
    /// Used blocks on this file system partition.
    #[serde(rename = "used")]
    pub used: Option<i32>,
}
