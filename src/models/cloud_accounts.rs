#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudAccounts {
    #[serde(rename = "accounts")]
    pub accounts: Option<Vec <crate::models::CloudAccountExtended>>,
    /// Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options).
    #[serde(rename = "resume")]
    pub resume: Option<String>,
}
