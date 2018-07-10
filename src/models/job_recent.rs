

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobRecent {
  #[serde(rename = "recent_jobs")]
  recent_jobs: Option<Vec<::models::JobJobExtended>>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

