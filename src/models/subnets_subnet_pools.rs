

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetsSubnetPools {
  #[serde(rename = "pools")]
  pools: Option<Vec<::models::SubnetsSubnetPoolsPool>>
}

