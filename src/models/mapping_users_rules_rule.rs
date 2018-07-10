#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersRulesRule {
    /// Specifies the operator to make rules on specified users or groups.
    #[serde(rename = "operator")]
    pub operator: Option<String>,
    /// Specifies the properties for user mapping rules.
    #[serde(rename = "options")]
    pub options: Option<::models::MappingUsersRulesRuleOptions>,
    ///
    #[serde(rename = "user1")]
    pub user1: Option<::models::MappingUsersRulesRuleUser2>,
    ///
    #[serde(rename = "user2")]
    pub user2: Option<::models::MappingUsersRulesRuleUser2>,
}
