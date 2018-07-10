

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultTopDirs {
  /// Change in directory ranking from result set comparison.
  #[serde(rename = "change")]
  change: Option<i32>,
  /// Directory access time enabled.
  #[serde(rename = "dir_atime_enabled")]
  dir_atime_enabled: bool,
  /// Directory listing.
  #[serde(rename = "dirs")]
  dirs: Vec<::models::ResultTopDirsDir>,
  /// Limit on number of top results.
  #[serde(rename = "top_n_max")]
  top_n_max: i32,
  /// Total count of directory results.
  #[serde(rename = "total_count")]
  total_count: i32
}

