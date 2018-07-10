

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportsScansReport {
  /// The number of bytes sent to the virus definition server to be scanned.
  #[serde(rename = "bytes_sent")]
  bytes_sent: Option<i32>,
  /// The length of time the job ran for.
  #[serde(rename = "duration")]
  duration: Option<i32>,
  #[serde(rename = "end")]
  end: Option<i32>,
  /// The number of files scanned.
  #[serde(rename = "files")]
  files: Option<i32>,
  /// A unique identifier for the report.
  #[serde(rename = "id")]
  id: Option<String>,
  /// The number of infections found.
  #[serde(rename = "infections")]
  infections: Option<i32>,
  #[serde(rename = "job_id")]
  job_id: Option<i32>,
  /// The id of the policy that this scan job executed.
  #[serde(rename = "policy_id")]
  policy_id: Option<String>,
  /// The cumulative size of the files scanned.
  #[serde(rename = "size")]
  size: Option<i32>,
  #[serde(rename = "start")]
  start: Option<i32>,
  /// The state of the job.
  #[serde(rename = "status")]
  status: Option<String>
}

