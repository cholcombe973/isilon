

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncJobCreateParams {
  /// The action to be taken by this job.
  #[serde(rename = "action")]
  action: Option<String>,
  /// The ID or Name of the policy
  #[serde(rename = "id")]
  id: String,
  /// Only valid for allow_write, and allow_write_revert; specify the desired logging level, will be stored in the logs for isi_migrate, defaults to 'info'.
  #[serde(rename = "log_level")]
  log_level: Option<String>,
  /// An optional snapshot to copy/sync from.
  #[serde(rename = "source_snapshot")]
  source_snapshot: Option<String>,
  /// Only valid for allow_write, and allow_write_revert; specify the desired workers per node, defaults to 3.
  #[serde(rename = "workers_per_node")]
  workers_per_node: Option<i32>
}

