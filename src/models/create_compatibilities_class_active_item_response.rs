

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompatibilitiesClassActiveItemResponse {
  /// A list of all merges that will occur given this compatibility operation
  #[serde(rename = "merges")]
  merges: Option<Vec<::models::CreateCompatibilitiesClassActiveItemResponseMerge>>,
  /// A string describing the effects of the compatibility operation.
  #[serde(rename = "message")]
  message: String,
  /// A list of all splits that will occur given this compatibility operation
  #[serde(rename = "splits")]
  splits: Option<Vec<::models::CreateCompatibilitiesClassActiveItemResponseSplit>>
}

