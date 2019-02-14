#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncPolicies {
    #[serde(rename = "policies")]
    pub policies: Option<Vec <crate::models::SyncPolicyExtended>>,
}
