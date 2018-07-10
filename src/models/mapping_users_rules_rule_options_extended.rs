

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersRulesRuleOptionsExtended {
  /// If true, and the rule was applied successfully, stop processing further.
  #[serde(rename = "break")]
  _break: Option<bool>,
  /// 
  #[serde(rename = "default_user")]
  default_user: Option<::models::MappingUsersRulesRuleOptionsDefaultUser>,
  /// If true, the primary GID and primary group SID should be copied to the existing credential.
  #[serde(rename = "group")]
  group: Option<bool>,
  /// If true, all additional identifiers should be copied to the existing credential.
  #[serde(rename = "groups")]
  groups: Option<bool>,
  /// If true, the primary UID and primary user SID should be copied to the existing credential.
  #[serde(rename = "user")]
  user: Option<bool>
}

