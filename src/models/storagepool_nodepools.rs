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
pub struct StoragepoolNodepools {
  #[serde(rename = "nodepools")]
  nodepools: Option<Vec<::models::StoragepoolNodepoolExtended>>
}

impl StoragepoolNodepools {
  pub fn new() -> StoragepoolNodepools {
    StoragepoolNodepools {
      nodepools: None
    }
  }

  pub fn set_nodepools(&mut self, nodepools: Vec<::models::StoragepoolNodepoolExtended>) {
    self.nodepools = Some(nodepools);
  }

  pub fn with_nodepools(mut self, nodepools: Vec<::models::StoragepoolNodepoolExtended>) -> StoragepoolNodepools {
    self.nodepools = Some(nodepools);
    self
  }

  pub fn nodepools(&self) -> Option<&Vec<::models::StoragepoolNodepoolExtended>> {
    self.nodepools.as_ref()
  }

  pub fn reset_nodepools(&mut self) {
    self.nodepools = None;
  }

}


