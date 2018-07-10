#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotPendingPendingItem {
    /// The system supplied unique ID used for sorting and paging.
    #[serde(rename = "id")]
    pub id: String,
    /// The /ifs path that will snapshotted.
    #[serde(rename = "path")]
    pub path: String,
    /// The name of the schedule used to create this snapshot.
    #[serde(rename = "schedule")]
    pub schedule: String,
    /// The system snapshot name formed from the schedule formate.
    #[serde(rename = "snapshot")]
    pub snapshot: String,
    /// The Unix Epoch time the snapshot will be created.
    #[serde(rename = "time")]
    pub time: i32,
}
