#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotChangelists {
    /// The system ID given to the changelist.
    #[serde(rename = "id")]
    pub id: String,
    /// The ID of the job which created the changelist.
    #[serde(rename = "job_id")]
    pub job_id: i32,
    /// Number of LIN entries in changelist.
    #[serde(rename = "num_entries")]
    pub num_entries: Option<i32>,
    /// Root path of all LINs in changelist.
    #[serde(rename = "root_path")]
    pub root_path: String,
    /// The lower snapid used to compute the changelist.
    #[serde(rename = "snap1")]
    pub snap1: i32,
    /// The higher snapid used to compute the changelist.
    #[serde(rename = "snap2")]
    pub snap2: i32,
    /// Status of changelist.
    #[serde(rename = "status")]
    pub status: String,
}
