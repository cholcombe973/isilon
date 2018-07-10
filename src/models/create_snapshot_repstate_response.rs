

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSnapshotRepstateResponse {
  /// The lower snapid used to compute the repstate.
  #[serde(rename = "snap1")]
  snap1: i32,
  /// The higher snapid used to compute the repstate.
  #[serde(rename = "snap2")]
  snap2: i32
}

