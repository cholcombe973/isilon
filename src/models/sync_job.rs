

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncJob {
  /// The state of the job.
  #[serde(rename = "state")]
  state: String
}

