#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DrivesDriveFirmwareUpdateNodeStatus {
    /// The number of drives that did not successfully complete firmware updates update on the node.
    #[serde(rename = "failed")]
    pub failed: Option<i32>,
    /// Time when drive firmware update finished on node.
    #[serde(rename = "finish_time")]
    pub finish_time: Option<String>,
    /// Number of drives remaining to be updated on node.
    #[serde(rename = "remaining")]
    pub remaining: Option<i32>,
    /// Time when drive firmware update started on node.
    #[serde(rename = "start_time")]
    pub start_time: Option<String>,
    /// Overall status of this nodes firmware updates.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// The number of drives that completed firmware updates on the node.
    #[serde(rename = "updated")]
    pub updated: Option<i32>,
}
