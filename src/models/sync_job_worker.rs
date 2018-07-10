

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncJobWorker {
  /// Whether there is a connection between the source and target.
  #[serde(rename = "connected")]
  connected: Option<bool>,
  /// The last time a network split occurred.
  #[serde(rename = "last_split")]
  last_split: Option<i32>,
  /// The last time the worker performed work.
  #[serde(rename = "last_work")]
  last_work: Option<i32>,
  /// The LIN being worked on.
  #[serde(rename = "lin")]
  lin: Option<i32>,
  /// The lnn the worker is assigned to run on.
  #[serde(rename = "lnn")]
  lnn: Option<i32>,
  /// The process ID of the worker.
  #[serde(rename = "process_id")]
  process_id: Option<i32>,
  /// The source host for this worker.
  #[serde(rename = "source_host")]
  source_host: Option<String>,
  /// The target host for this worker.
  #[serde(rename = "target_host")]
  target_host: Option<String>,
  /// The ID of the worker.
  #[serde(rename = "worker_id")]
  worker_id: Option<i32>
}

