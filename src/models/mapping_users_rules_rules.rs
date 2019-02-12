#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersRulesRules {
    /// Specifies the default UNIX user information that can be applied if the final credentials do not have valid UID and GID information.
    #[serde(rename = "parameters")]
    pub parameters: Option <crate::models::MappingUsersRulesRulesParameters>,
    /// Specifies the list of user mapping rules.
    #[serde(rename = "rules")]
    pub rules: Option<Vec <crate::models::MappingUsersRulesRule>>,
}
