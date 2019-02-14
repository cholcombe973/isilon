#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudPools {
    #[serde(rename = "pools")]
    pub pools: Option<Vec <crate::models::CloudPoolExtended>>,
    /// Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options).
    #[serde(rename = "resume")]
    pub resume: Option<String>,
}
