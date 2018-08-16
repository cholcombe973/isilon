#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatisticsJobNodeIoWrite {
    /// The number of bytes recently written by this job on this node.
    #[serde(rename = "bytes")]
    pub bytes: u64,
    /// The number of write operations recently performed by this job on this node.
    #[serde(rename = "ops")]
    pub ops: u64,
}
