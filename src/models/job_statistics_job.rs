

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatisticsJob {
  /// The job ID.
  #[serde(rename = "job_id")]
  job_id: i32,
  #[serde(rename = "nodes")]
  nodes: Vec<::models::JobStatisticsJobNode>,
  /// The current phase of the job.
  #[serde(rename = "phase")]
  phase: i32,
  /// The number of nodes participating in the job.
  #[serde(rename = "total_nodes")]
  total_nodes: i32
}

