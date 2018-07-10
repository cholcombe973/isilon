

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudJobFiles {
  /// The file filtering logic to find files for this job
  #[serde(rename = "file_matching_pattern")]
  file_matching_pattern: Option<::models::Empty>,
  #[serde(rename = "names")]
  names: Option<Vec<::models::CloudJobFilesName>>,
  /// The total number of files addressed by this job
  #[serde(rename = "total")]
  total: Option<i32>,
  /// The number of canceled files
  #[serde(rename = "total_canceled")]
  total_canceled: Option<i32>,
  /// The number of files which failed
  #[serde(rename = "total_failed")]
  total_failed: Option<i32>,
  /// The number of files pending action
  #[serde(rename = "total_pending")]
  total_pending: Option<i32>,
  /// The number of files currently being processed
  #[serde(rename = "total_processing")]
  total_processing: Option<i32>,
  /// The total number of files successfully completed
  #[serde(rename = "total_succeeded")]
  total_succeeded: Option<i32>
}

