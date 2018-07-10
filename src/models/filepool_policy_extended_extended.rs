

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilepoolPolicyExtendedExtended {
  /// A list of actions to be taken for matching files
  #[serde(rename = "actions")]
  actions: Option<Vec<::models::FilepoolPolicyActionExtended>>,
  /// The order in which this policy should be applied (relative to other policies)
  #[serde(rename = "apply_order")]
  apply_order: Option<i32>,
  /// The guid assigned to the cluster on which the account was created
  #[serde(rename = "birth_cluster_id")]
  birth_cluster_id: Option<String>,
  /// A description for this policy
  #[serde(rename = "description")]
  description: Option<String>,
  /// The file matching rules for this policy
  #[serde(rename = "file_matching_pattern")]
  file_matching_pattern: Option<::models::FilepoolPolicyFileMatchingPattern>,
  /// A unique identifier for this policy
  #[serde(rename = "id")]
  id: Option<i32>,
  /// A unique name for this policy
  #[serde(rename = "name")]
  name: Option<String>,
  /// Indicates whether this policy is in a good state (\"OK\") or disabled (\"disabled\")
  #[serde(rename = "state")]
  state: Option<String>,
  /// Gives further information to describe the state of this policy
  #[serde(rename = "state_details")]
  state_details: Option<String>
}

