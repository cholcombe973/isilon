#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticsGatherStatusGatherStatus {
    /// The current status of the process
    #[serde(rename = "Active_Status")]
    pub active_status: String,
    /// The previous status of the process
    #[serde(rename = "Last_Status")]
    pub last_status: String,
}
