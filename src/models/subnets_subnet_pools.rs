/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetsSubnetPools {
  #[serde(rename = "pools")]
  pools: Option<Vec<::models::SubnetsSubnetPoolsPool>>
}

impl SubnetsSubnetPools {
  pub fn new() -> SubnetsSubnetPools {
    SubnetsSubnetPools {
      pools: None
    }
  }

  pub fn set_pools(&mut self, pools: Vec<::models::SubnetsSubnetPoolsPool>) {
    self.pools = Some(pools);
  }

  pub fn with_pools(mut self, pools: Vec<::models::SubnetsSubnetPoolsPool>) -> SubnetsSubnetPools {
    self.pools = Some(pools);
    self
  }

  pub fn pools(&self) -> Option<&Vec<::models::SubnetsSubnetPoolsPool>> {
    self.pools.as_ref()
  }

  pub fn reset_pools(&mut self) {
    self.pools = None;
  }

}


