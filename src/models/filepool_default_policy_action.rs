#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilepoolDefaultPolicyAction {
    /// Varies according to action_type
    #[serde(rename = "action_param")]
    pub action_param: String,
    #[serde(rename = "action_type")]
    pub action_type: String,
}
