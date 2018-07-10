

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobPolicies {
  #[serde(rename = "policies")]
  policies: Option<Vec<::models::JobPolicyExtended>>
}

