#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolsPoolScResumeNode {
    /// Logical node numbers of the nodes suspended/resumed.
    #[serde(rename = "lnn")]
    pub lnn: Vec<i32>,
}
