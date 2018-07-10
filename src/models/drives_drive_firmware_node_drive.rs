

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DrivesDriveFirmwareNodeDrive {
  /// Numerical representation of this drive's bay.
  #[serde(rename = "baynum")]
  baynum: Option<i32>,
  /// This drive's current firmware revision
  #[serde(rename = "current_firmware")]
  current_firmware: Option<String>,
  /// This drive's desired firmware revision.
  #[serde(rename = "desired_firmware")]
  desired_firmware: Option<String>,
  /// This drive's device name.
  #[serde(rename = "devname")]
  devname: Option<String>,
  /// This drive's logical drive number in IFS.
  #[serde(rename = "lnum")]
  lnum: Option<i32>,
  /// String representation of this drive's physical location.
  #[serde(rename = "locnstr")]
  locnstr: Option<String>,
  /// This drive's manufacturer and model.
  #[serde(rename = "model")]
  model: Option<String>
}

