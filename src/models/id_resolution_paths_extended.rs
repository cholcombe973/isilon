#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdResolutionPathsExtended {
    #[serde(rename = "paths")]
    pub paths: Option<Vec <crate::models::IdResolutionPath>>,
    #[serde(rename = "errors")]
    pub errors: Option<Vec <crate::models::IdResolutionPathsError>>,
    /// Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options).
    #[serde(rename = "resume")]
    pub resume: Option<String>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
