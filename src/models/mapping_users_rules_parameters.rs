#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersRulesParameters {
    ///
    #[serde(rename = "default_unix_user")]
    pub default_unix_user: Option <crate::models::MappingUsersRulesParametersDefaultUnixUser>,
}
