#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotSnapshots {
    #[serde(rename = "snapshots")]
    pub snapshots: Option<Vec<::models::SnapshotSnapshotExtended>>,
}
