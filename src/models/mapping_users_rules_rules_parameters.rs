

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersRulesRulesParameters {
  /// 
  #[serde(rename = "default_unix_user")]
  default_unix_user: Option<::models::MappingUsersRulesRuleUser2>
}

