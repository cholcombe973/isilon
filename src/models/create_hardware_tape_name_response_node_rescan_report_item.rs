

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHardwareTapeNameResponseNodeRescanReportItem {
  /// device name
  #[serde(rename = "devicename")]
  devicename: Option<String>,
  /// device driver path
  #[serde(rename = "path")]
  path: Option<String>,
  /// device product name
  #[serde(rename = "product")]
  product: Option<String>,
  /// device serial:L number
  #[serde(rename = "serial")]
  serial: Option<String>,
  /// device change status
  #[serde(rename = "status_report")]
  status_report: Option<String>,
  /// device node world wide name
  #[serde(rename = "wwnn")]
  wwnn: Option<String>
}

