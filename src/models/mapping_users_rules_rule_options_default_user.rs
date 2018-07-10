#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersRulesRuleOptionsDefaultUser {
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    #[serde(rename = "user")]
    pub user: String,
}
