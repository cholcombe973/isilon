/// MappingUsersRulesExtended : Specifies the properties for user mapping rules.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersRulesExtended {
    /// Specifies the default UNIX user information that can be applied if the final credentials do not have valid UID and GID information.
    #[serde(rename = "parameters")]
    pub parameters: Option<::models::MappingUsersRulesParameters>,
    /// Specifies the list of user mapping rules.
    #[serde(rename = "rules")]
    pub rules: Option<Vec<::models::MappingUsersRulesRuleExtended>>,
}
