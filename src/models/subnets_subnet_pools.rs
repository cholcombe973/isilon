#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetsSubnetPools {
    #[serde(rename = "pools")]
    pub pools: Option<Vec <crate::models::SubnetsSubnetPoolsPool>>,
}
