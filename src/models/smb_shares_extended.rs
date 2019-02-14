#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbSharesExtended {
    #[serde(rename = "shares")]
    pub shares: Option<Vec <crate::models::SmbShareExtended>>,
    /// An identifier for a set of shares.
    #[serde(rename = "digest")]
    pub digest: Option<String>,
    /// Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options).
    #[serde(rename = "resume")]
    pub resume: Option<String>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
