

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterFirmwareStatusNodeDevice {
  /// The name of the device.
  #[serde(rename = "device")]
  device: Option<String>,
  /// Is the firmware up-to-date for this component.
  #[serde(rename = "mismatch")]
  mismatch: Option<bool>,
  /// The target firmware version.
  #[serde(rename = "target_version")]
  target_version: Option<String>,
  /// The device type.
  #[serde(rename = "type")]
  _type: Option<String>,
  /// The current state of the firmware upgrade for this component. One of the following values: 'queued', 'upgrading', 'upgraded', 'error'. or 'null'.'null' indicates that the upgrade status is unknown.
  #[serde(rename = "upgrade_status")]
  upgrade_status: Option<String>,
  /// The current firmware version.
  #[serde(rename = "version")]
  version: Option<String>
}

