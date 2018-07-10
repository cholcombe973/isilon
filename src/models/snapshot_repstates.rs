#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotRepstates {
    /// The system ID given to the repstate.
    #[serde(rename = "id")]
    pub id: String,
    /// The lower snapid used to compute the repstate.
    #[serde(rename = "snap1")]
    pub snap1: i32,
    /// The higher snapid used to compute the repstate.
    #[serde(rename = "snap2")]
    pub snap2: i32,
}
