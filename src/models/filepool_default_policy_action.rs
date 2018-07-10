

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

