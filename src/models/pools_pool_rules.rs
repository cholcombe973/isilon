#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolsPoolRules {
    #[serde(rename = "rules")]
    pub rules: Option<Vec<::models::PoolsPoolRulesRule>>,
}
