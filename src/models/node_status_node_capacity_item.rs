#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatusNodeCapacityItem {
    /// Total device storage bytes.
    #[serde(rename = "bytes")]
    pub bytes: Option<u64>,
    /// Total device count.
    #[serde(rename = "count")]
    pub count: Option<i32>,
    /// Device type.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
