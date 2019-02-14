#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersRulesRuleOptions {
    /// If true, and the rule was applied successfully, stop processing further.
    #[serde(rename = "break")]
    pub _break: Option<bool>,
    ///
    #[serde(rename = "default_user")]
    pub default_user: Option <crate::models::MappingUsersRulesRuleUser2>,
    /// If true, the primary GID and primary group SID should be copied to the existing credential.
    #[serde(rename = "group")]
    pub group: Option<bool>,
    /// If true, all additional identifiers should be copied to the existing credential.
    #[serde(rename = "groups")]
    pub groups: Option<bool>,
    /// If true, the primary UID and primary user SID should be copied to the existing credential.
    #[serde(rename = "user")]
    pub user: Option<bool>,
}
