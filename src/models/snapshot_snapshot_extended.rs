#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotSnapshotExtended {
    /// Alias name to create for this snapshot. If null, remove any alias.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// The Unix Epoch time the snapshot will expire and be eligible for automatic deletion.
    #[serde(rename = "expires")]
    pub expires: Option<i32>,
    /// The user or system supplied snapshot name. This will be null for snapshots pending delete.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The Unix Epoch time the snapshot was created.
    #[serde(rename = "created")]
    pub created: i32,
    /// True if the snapshot has one or more locks present see, see the locks subresource of a snapshot for a list of locks.
    #[serde(rename = "has_locks")]
    pub has_locks: bool,
    /// The system ID given to the snapshot. This is useful for tracking the status of delete pending snapshots.
    #[serde(rename = "id")]
    pub id: i32,
    /// The /ifs path snapshotted.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// Percentage of /ifs used for storing this snapshot.
    #[serde(rename = "pct_filesystem")]
    pub pct_filesystem: f32,
    /// Percentage of configured snapshot reserved used for storing this snapshot.
    #[serde(rename = "pct_reserve")]
    pub pct_reserve: f32,
    /// The name of the schedule used to create this snapshot, if applicable.
    #[serde(rename = "schedule")]
    pub schedule: Option<String>,
    /// The amount of shadow bytes referred to by this snapshot.
    #[serde(rename = "shadow_bytes")]
    pub shadow_bytes: i32,
    /// The amount of storage in bytes used to store this snapshot.
    #[serde(rename = "size")]
    pub size: i32,
    /// Snapshot state.
    #[serde(rename = "state")]
    pub state: String,
    /// The ID of the snapshot pointed to if this is an alias.
    #[serde(rename = "target_id")]
    pub target_id: Option<i32>,
    /// The name of the snapshot pointed to if this is an alias.
    #[serde(rename = "target_name")]
    pub target_name: Option<String>,
}
