/// FilepoolDefaultPolicy : A default filepool policy object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilepoolDefaultPolicy {
    /// A default filepool policy object
    #[serde(rename = "default-policy")]
    pub default_policy: Option<crate::models::FilepoolDefaultPolicyDefaultPolicy>,
}
