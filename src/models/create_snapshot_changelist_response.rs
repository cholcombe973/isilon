

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSnapshotChangelistResponse {
  /// The ID of the job which created the changelist.
  #[serde(rename = "job_id")]
  job_id: i32,
  /// Number of LIN entries in changelist.
  #[serde(rename = "num_entries")]
  num_entries: Option<i32>,
  /// Root path of all LINs in changelist.
  #[serde(rename = "root_path")]
  root_path: String,
  /// The lower snapid used to compute the changelist.
  #[serde(rename = "snap1")]
  snap1: i32,
  /// The higher snapid used to compute the changelist.
  #[serde(rename = "snap2")]
  snap2: i32,
  /// Status of changelist.
  #[serde(rename = "status")]
  status: String
}

