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
pub struct FilepoolDefaultPolicyAction {
  /// Varies according to action_type
  #[serde(rename = "action_param")]
  action_param: String,
  #[serde(rename = "action_type")]
  action_type: String
}

impl FilepoolDefaultPolicyAction {
  pub fn new(action_param: String, action_type: String) -> FilepoolDefaultPolicyAction {
    FilepoolDefaultPolicyAction {
      action_param: action_param,
      action_type: action_type
    }
  }

  pub fn set_action_param(&mut self, action_param: String) {
    self.action_param = action_param;
  }

  pub fn with_action_param(mut self, action_param: String) -> FilepoolDefaultPolicyAction {
    self.action_param = action_param;
    self
  }

  pub fn action_param(&self) -> &String {
    &self.action_param
  }


  pub fn set_action_type(&mut self, action_type: String) {
    self.action_type = action_type;
  }

  pub fn with_action_type(mut self, action_type: String) -> FilepoolDefaultPolicyAction {
    self.action_type = action_type;
    self
  }

  pub fn action_type(&self) -> &String {
    &self.action_type
  }


}


