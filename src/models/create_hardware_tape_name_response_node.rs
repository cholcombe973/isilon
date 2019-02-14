#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHardwareTapeNameResponseNode {
    /// Error message, if the HTTP status returned from this node was not 200.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Node ID of the node reporting this information.
    #[serde(rename = "id")]
    pub id: Option<i32>,
    /// Logical node number of the node reporting this information.
    #[serde(rename = "lnn")]
    pub lnn: Option<i32>,
    /// A list of device rescan status
    #[serde(rename = "rescan_report")]
    pub rescan_report: Option<Vec <crate::models::CreateHardwareTapeNameResponseNodeRescanReportItem>>,
    /// Status of the HTTP response from this node if not 200.  If 200, this field does not appear.
    #[serde(rename = "status")]
    pub status: Option<i32>,
}
