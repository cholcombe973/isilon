#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotScheduleExtended {
    /// Alias name to create for each snapshot.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Time in seconds added to creation time to construction expiration time.
    #[serde(rename = "duration")]
    pub duration: Option<i32>,
    /// The system ID given to the schedule.
    #[serde(rename = "id")]
    pub id: Option<i32>,
    /// The schedule name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Unix Epoch time of next snapshot to be created.
    #[serde(rename = "next_run")]
    pub next_run: Option<i32>,
    /// Formatted name (see pattern) of next snapshot to be created.
    #[serde(rename = "next_snapshot")]
    pub next_snapshot: Option<String>,
    /// The /ifs path snapshotted.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// Pattern expanded with strftime to create snapshot name.
    #[serde(rename = "pattern")]
    pub pattern: Option<String>,
    /// The isidate compatible natural language description of the schedule.
    #[serde(rename = "schedule")]
    pub schedule: Option<String>,
}
