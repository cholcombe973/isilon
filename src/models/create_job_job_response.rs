

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateJobJobResponse {
  /// The ID of the job.
  #[serde(rename = "id")]
  id: i32
}

