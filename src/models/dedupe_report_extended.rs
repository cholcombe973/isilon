

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DedupeReportExtended {
  /// The amount of space the directory trees under this job's paths now take up, compared to what they would take up if not deduplicated (0 ~ 100).
  #[serde(rename = "dedupe_percent")]
  dedupe_percent: Option<String>,
  /// The amount of time in seconds it took to run this job.
  #[serde(rename = "elapsed_time")]
  elapsed_time: Option<i32>,
  /// An unique identifier for this report.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// The job id this report refers to.
  #[serde(rename = "job_id")]
  job_id: Option<i32>,
  /// The type of dedupe job this report refers to.
  #[serde(rename = "job_type")]
  job_type: Option<String>,
  /// A list of report entries for this dedupe job.
  #[serde(rename = "reports")]
  reports: Option<Vec<::models::DedupeReport>>
}

