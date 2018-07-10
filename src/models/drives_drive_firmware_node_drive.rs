#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DrivesDriveFirmwareNodeDrive {
    /// Numerical representation of this drive's bay.
    #[serde(rename = "baynum")]
    pub baynum: Option<i32>,
    /// This drive's current firmware revision
    #[serde(rename = "current_firmware")]
    pub current_firmware: Option<String>,
    /// This drive's desired firmware revision.
    #[serde(rename = "desired_firmware")]
    pub desired_firmware: Option<String>,
    /// This drive's device name.
    #[serde(rename = "devname")]
    pub devname: Option<String>,
    /// This drive's logical drive number in IFS.
    #[serde(rename = "lnum")]
    pub lnum: Option<i32>,
    /// String representation of this drive's physical location.
    #[serde(rename = "locnstr")]
    pub locnstr: Option<String>,
    /// This drive's manufacturer and model.
    #[serde(rename = "model")]
    pub model: Option<String>,
}
