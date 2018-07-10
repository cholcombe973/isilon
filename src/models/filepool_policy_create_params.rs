

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilepoolPolicyCreateParams {
  /// A list of actions to be taken for matching files
  #[serde(rename = "actions")]
  actions: Option<Vec<::models::FilepoolDefaultPolicyAction>>,
  /// The order in which this policy should be applied (relative to other policies)
  #[serde(rename = "apply_order")]
  apply_order: Option<i32>,
  /// A description for this policy
  #[serde(rename = "description")]
  description: Option<String>,
  /// The file matching rules for this policy
  #[serde(rename = "file_matching_pattern")]
  file_matching_pattern: ::models::FilepoolPolicyFileMatchingPattern,
  /// A unique name for this policy
  #[serde(rename = "name")]
  name: String
}

