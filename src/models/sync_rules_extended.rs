#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRulesExtended {
    /// Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options).
    #[serde(rename = "resume")]
    pub resume: Option<String>,
    #[serde(rename = "rules")]
    pub rules: Vec <crate::models::SyncRuleExtendedExtended>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: i32,
}
