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
pub struct NodeSledsNodeSled {
  /// Boolean to indicate whether or not the sled is safe to remove.
  #[serde(rename = "is_removeable")]
  is_removeable: bool,
  /// The sled letter which OneFS uses to refer to this sled in the node.
  #[serde(rename = "sled_letter")]
  sled_letter: String,
  /// The state of physical presence of a sled.
  #[serde(rename = "sled_state")]
  sled_state: String
}

impl NodeSledsNodeSled {
  pub fn new(is_removeable: bool, sled_letter: String, sled_state: String) -> NodeSledsNodeSled {
    NodeSledsNodeSled {
      is_removeable: is_removeable,
      sled_letter: sled_letter,
      sled_state: sled_state
    }
  }

  pub fn set_is_removeable(&mut self, is_removeable: bool) {
    self.is_removeable = is_removeable;
  }

  pub fn with_is_removeable(mut self, is_removeable: bool) -> NodeSledsNodeSled {
    self.is_removeable = is_removeable;
    self
  }

  pub fn is_removeable(&self) -> &bool {
    &self.is_removeable
  }


  pub fn set_sled_letter(&mut self, sled_letter: String) {
    self.sled_letter = sled_letter;
  }

  pub fn with_sled_letter(mut self, sled_letter: String) -> NodeSledsNodeSled {
    self.sled_letter = sled_letter;
    self
  }

  pub fn sled_letter(&self) -> &String {
    &self.sled_letter
  }


  pub fn set_sled_state(&mut self, sled_state: String) {
    self.sled_state = sled_state;
  }

  pub fn with_sled_state(mut self, sled_state: String) -> NodeSledsNodeSled {
    self.sled_state = sled_state;
    self
  }

  pub fn sled_state(&self) -> &String {
    &self.sled_state
  }


}


