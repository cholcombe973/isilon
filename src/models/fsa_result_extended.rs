

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FsaResultExtended {
  /// True if the result is pinned to prevent automatic removal.
  #[serde(rename = "pinned")]
  pinned: bool,
  /// Unix Epoch time of start of results collection job.
  #[serde(rename = "begin_time")]
  begin_time: i32,
  /// Path to results database.
  #[serde(rename = "content_path")]
  content_path: Option<String>,
  /// Resource to call with DELETE to remove results..
  #[serde(rename = "delete_link")]
  delete_link: Option<String>,
  /// Unix Epoch time of end of results collection job.
  #[serde(rename = "end_time")]
  end_time: i32,
  /// State of the result set.
  #[serde(rename = "fsa_state")]
  fsa_state: String,
  /// The system generated result set ID.
  #[serde(rename = "id")]
  id: i32,
  /// State information about the FSA Job.
  #[serde(rename = "job_state")]
  job_state: Vec<String>,
  /// Resource to call to get result properties.
  #[serde(rename = "properties_link")]
  properties_link: String,
  /// Size of the result set database in bytes.
  #[serde(rename = "size")]
  size: i32,
  /// FSA version used to create result set.
  #[serde(rename = "version")]
  version: i32
}

