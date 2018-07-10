

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRules {
  #[serde(rename = "rules")]
  rules: Option<Vec<::models::SyncRuleExtended>>
}

