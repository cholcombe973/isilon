#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodePartitions {
    /// Count of how many partitions are included.
    #[serde(rename = "count")]
    pub count: Option<i32>,
    /// Partition information.
    #[serde(rename = "partitions")]
    pub partitions: Option<Vec <crate::models::NodePartitionsNodePartition>>,
}
