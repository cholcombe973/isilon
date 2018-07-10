

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotScheduleCreateParams {
  /// Alias name to create for each snapshot.
  #[serde(rename = "alias")]
  alias: Option<String>,
  /// Time in seconds added to creation time to construction expiration time.
  #[serde(rename = "duration")]
  duration: Option<i32>,
  /// The schedule name.
  #[serde(rename = "name")]
  name: String,
  /// The /ifs path snapshotted.
  #[serde(rename = "path")]
  path: String,
  /// Pattern expanded with strftime to create snapshot names.
  #[serde(rename = "pattern")]
  pattern: String,
  /// The isidate compatible natural language description of the schedule.
  #[serde(rename = "schedule")]
  schedule: String
}

