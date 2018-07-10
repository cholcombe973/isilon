

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilepoolDefaultPolicyDefaultPolicy {
  /// A list of actions to be taken for matching files
  #[serde(rename = "actions")]
  actions: Option<Vec<::models::FilepoolDefaultPolicyDefaultPolicyAction>>
}

