#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodesOnefsVersion {
    #[serde(rename = "bugfix")]
    pub bugfix: Option<i32>,
    #[serde(rename = "maintenance")]
    pub maintenance: Option<i32>,
    #[serde(rename = "major")]
    pub major: Option<i32>,
    #[serde(rename = "minor")]
    pub minor: Option<i32>,
    /// hex representation of the OneFS version integer.
    #[serde(rename = "version")]
    pub version: Option<String>,
}
