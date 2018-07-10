

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersRules {
  /// Specifies the properties for user mapping rules.
  #[serde(rename = "rules")]
  rules: Option<::models::MappingUsersRulesRules>
}

