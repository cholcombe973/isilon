#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetsSubnetPoolRange {
    /// High IP
    #[serde(rename = "high")]
    pub high: String,
    /// Low IP
    #[serde(rename = "low")]
    pub low: String,
}
