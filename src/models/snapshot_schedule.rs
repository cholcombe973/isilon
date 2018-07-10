

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotSchedule {
  /// Alias name to create for each snapshot.
  #[serde(rename = "alias")]
  alias: Option<String>,
  /// Time in seconds added to creation time to construction expiration time.
  #[serde(rename = "duration")]
  duration: Option<i32>,
  /// The schedule name.
  #[serde(rename = "name")]
  name: Option<String>,
  /// The /ifs path snapshotted.
  #[serde(rename = "path")]
  path: Option<String>,
  /// Pattern expanded with strftime to create snapshot names.
  #[serde(rename = "pattern")]
  pattern: Option<String>,
  /// The isidate compatible natural language description of the schedule.
  #[serde(rename = "schedule")]
  schedule: Option<String>
}

