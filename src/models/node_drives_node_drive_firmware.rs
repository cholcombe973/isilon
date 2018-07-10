#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDrivesNodeDriveFirmware {
    /// This drive's current firmware revision
    #[serde(rename = "current_firmware")]
    pub current_firmware: Option<String>,
    /// This drive's desired firmware revision.
    #[serde(rename = "desired_firmware")]
    pub desired_firmware: Option<String>,
}
