#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHardwareTapeNameResponseNodeRescanReportItem {
    /// device name
    #[serde(rename = "devicename")]
    pub devicename: Option<String>,
    /// device driver path
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// device product name
    #[serde(rename = "product")]
    pub product: Option<String>,
    /// device serial:L number
    #[serde(rename = "serial")]
    pub serial: Option<String>,
    /// device change status
    #[serde(rename = "status_report")]
    pub status_report: Option<String>,
    /// device node world wide name
    #[serde(rename = "wwnn")]
    pub wwnn: Option<String>,
}
