

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersRulesRuleUser2 {
  #[serde(rename = "domain")]
  domain: Option<String>,
  #[serde(rename = "user")]
  user: Option<String>
}

