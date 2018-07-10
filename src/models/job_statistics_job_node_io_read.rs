#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatisticsJobNodeIoRead {
    /// The number of bytes recently read by this job on this node.
    #[serde(rename = "bytes")]
    pub bytes: i32,
    /// The number of read operations recently performed by this job on this node.
    #[serde(rename = "ops")]
    pub ops: i32,
}
