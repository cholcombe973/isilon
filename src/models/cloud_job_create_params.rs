
/// CloudJobCreateParams : A cloud job for archiving or recalling files or restoring COI

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudJobCreateParams {
  /// The names of accounts for COI restore
  #[serde(rename = "accounts")]
  accounts: Option<Vec<String>>,
  /// Directories addressed by this job
  #[serde(rename = "directories")]
  directories: Option<Vec<String>>,
  /// The new expiration date in seconds
  #[serde(rename = "expiration_date")]
  expiration_date: Option<i32>,
  /// The file filtering logic to find files for this job. (Only applicable for 'recall' jobs)
  #[serde(rename = "file_matching_pattern")]
  file_matching_pattern: Option<::models::Empty>,
  /// Filenames addressed by this job
  #[serde(rename = "files")]
  files: Option<Vec<String>>,
  /// The name of an existing cloudpool policy to apply to this job. (Only applicable for 'archive' jobs)
  #[serde(rename = "policy")]
  policy: Option<String>,
  /// The type of cloud action to be performed by this job.
  #[serde(rename = "type")]
  _type: String
}

