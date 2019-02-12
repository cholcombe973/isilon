#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompatibilitiesClassActiveItemResponse {
    /// A list of all merges that will occur given this compatibility operation
    #[serde(rename = "merges")]
    pub merges: Option<Vec <crate::models::CreateCompatibilitiesClassActiveItemResponseMerge>>,
    /// A string describing the effects of the compatibility operation.
    #[serde(rename = "message")]
    pub message: String,
    /// A list of all splits that will occur given this compatibility operation
    #[serde(rename = "splits")]
    pub splits: Option<Vec <crate::models::CreateCompatibilitiesClassActiveItemResponseSplit>>,
}
