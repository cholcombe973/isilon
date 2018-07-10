
/// HardwareTapes : Get list Tape and Changer devices

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareTapes {
  /// Information of Tape/MC device
  #[serde(rename = "devices")]
  devices: Option<::models::HardwareTapesDevices>,
  /// Resume string returned by previous query.
  #[serde(rename = "resume")]
  resume: Option<String>,
  /// The number of devices
  #[serde(rename = "total")]
  total: Option<i32>
}

