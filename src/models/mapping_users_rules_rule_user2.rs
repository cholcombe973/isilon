#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersRulesRuleUser2 {
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    #[serde(rename = "user")]
    pub user: Option<String>,
}
