

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersRulesParameters {
  /// 
  #[serde(rename = "default_unix_user")]
  default_unix_user: Option<::models::MappingUsersRulesParametersDefaultUnixUser>
}

