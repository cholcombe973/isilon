#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilepoolDefaultPolicyExtended {
    #[serde(rename = "actions")]
    pub actions: Option<Vec<::models::FilepoolDefaultPolicyAction>>,
}
