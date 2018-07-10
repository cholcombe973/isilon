#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodePartitionsNode {
    /// Count of how many partitions are included.
    #[serde(rename = "count")]
    pub count: Option<i32>,
    /// Error message, if the HTTP status returned from this node was not 200.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Node ID of the node reporting this information.
    #[serde(rename = "id")]
    pub id: Option<i32>,
    /// Logical node number of the node reporting this information.
    #[serde(rename = "lnn")]
    pub lnn: Option<i32>,
    /// Partition information.
    #[serde(rename = "partitions")]
    pub partitions: Option<Vec<::models::NodePartitionsNodePartition>>,
    /// Status of the HTTP response from this node if not 200.  If 200, this field does not appear.
    #[serde(rename = "status")]
    pub status: Option<i32>,
}
