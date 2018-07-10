

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdResolutionPathsExtended {
  #[serde(rename = "paths")]
  paths: Option<Vec<::models::IdResolutionPath>>,
  #[serde(rename = "errors")]
  errors: Option<Vec<::models::IdResolutionPathsError>>,
  /// Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options).
  #[serde(rename = "resume")]
  resume: Option<String>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

