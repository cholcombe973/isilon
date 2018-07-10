

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AntivirusPolicies {
  #[serde(rename = "policies")]
  policies: Option<Vec<::models::AntivirusPolicyExtended>>
}

