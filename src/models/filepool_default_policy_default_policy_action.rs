

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilepoolDefaultPolicyDefaultPolicyAction {
  /// Varies according to action_type
  #[serde(rename = "action_param")]
  action_param: Option<String>,
  #[serde(rename = "action_type")]
  action_type: String
}

