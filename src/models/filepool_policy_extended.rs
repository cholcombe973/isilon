#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilepoolPolicyExtended {
    /// A list of actions to be taken for matching files
    #[serde(rename = "actions")]
    pub actions: Option<Vec <crate::models::FilepoolPolicyAction>>,
    /// The order in which this policy should be applied (relative to other policies)
    #[serde(rename = "apply_order")]
    pub apply_order: Option<i32>,
    /// The guid assigned to the cluster on which the account was created
    #[serde(rename = "birth_cluster_id")]
    pub birth_cluster_id: Option<String>,
    /// A description for this policy
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The file matching rules for this policy
    #[serde(rename = "file_matching_pattern")]
    pub file_matching_pattern: Option <crate::models::FilepoolPolicyFileMatchingPattern>,
    /// A unique identifier for this policy
    #[serde(rename = "id")]
    pub id: Option<i32>,
    /// A unique name for this policy
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Indicates whether this policy is in a good state (\"OK\") or disabled (\"disabled\")
    #[serde(rename = "state")]
    pub state: Option<String>,
    /// Gives further information to describe the state of this policy
    #[serde(rename = "state_details")]
    pub state_details: Option<String>,
}
