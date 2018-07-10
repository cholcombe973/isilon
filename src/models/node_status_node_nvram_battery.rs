

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatusNodeNvramBattery {
  /// The current status color of the NVRAM battery.
  #[serde(rename = "color")]
  color: Option<String>,
  /// Identifying index for the NVRAM battery.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// The current status message of the NVRAM battery.
  #[serde(rename = "status")]
  status: Option<String>,
  /// The current voltage of the NVRAM battery.
  #[serde(rename = "voltage")]
  voltage: Option<String>
}

