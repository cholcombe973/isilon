#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersRulesRuleExtended {
    /// Specifies the operator to make rules on specified users or groups.
    #[serde(rename = "operator")]
    pub operator: Option<String>,
    /// Specifies the properties for user mapping rules.
    #[serde(rename = "options")]
    pub options: Option<::models::MappingUsersRulesRuleOptionsExtended>,
    ///
    #[serde(rename = "user1")]
    pub user1: ::models::MappingUsersRulesRuleUser1,
    ///
    #[serde(rename = "user2")]
    pub user2: Option<::models::MappingUsersRulesRuleUser2Extended>,
}
