#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaQuotaUsage {
    /// Number of inodes (filesystem entities) used by governed data.
    #[serde(rename = "inodes")]
    pub inodes: i32,
    /// Apparent bytes used by governed data.
    #[serde(rename = "logical")]
    pub logical: i32,
    /// Bytes used for governed data and filesystem overhead.
    #[serde(rename = "physical")]
    pub physical: i32,
}
