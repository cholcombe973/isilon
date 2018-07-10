#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilepoolPolicyAction {
    /// Varies according to action_type
    #[serde(rename = "action_param")]
    pub action_param: Option<String>,
    #[serde(rename = "action_type")]
    pub action_type: String,
}
