

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatusNodeNvram {
  /// This node's NVRAM battery status information.
  #[serde(rename = "batteries")]
  batteries: Option<Vec<::models::NodeStatusNodeNvramBattery>>,
  /// This node's NVRAM battery count.
  #[serde(rename = "battery_count")]
  battery_count: Option<i32>,
  /// This node's NVRAM battery charge status, as a color.
  #[serde(rename = "charge_status")]
  charge_status: Option<String>,
  /// This node's NVRAM battery charge status, as a number.
  #[serde(rename = "charge_status_number")]
  charge_status_number: Option<i32>,
  /// This node's NVRAM device name with path.
  #[serde(rename = "device")]
  device: Option<String>,
  /// This node has NVRAM.
  #[serde(rename = "present")]
  present: Option<bool>,
  /// This node has NVRAM with flash storage.
  #[serde(rename = "present_flash")]
  present_flash: Option<bool>,
  /// The size of the NVRAM, in bytes.
  #[serde(rename = "present_size")]
  present_size: Option<i32>,
  /// This node's NVRAM type.
  #[serde(rename = "present_type")]
  present_type: Option<String>,
  /// This node's current ship mode state for NVRAM batteries.
  #[serde(rename = "ship_mode")]
  ship_mode: Option<i32>,
  /// This node supports NVRAM.
  #[serde(rename = "supported")]
  supported: Option<bool>,
  /// This node supports NVRAM with flash storage.
  #[serde(rename = "supported_flash")]
  supported_flash: Option<bool>,
  /// The maximum size of the NVRAM, in bytes.
  #[serde(rename = "supported_size")]
  supported_size: Option<i32>,
  /// This node's supported NVRAM type.
  #[serde(rename = "supported_type")]
  supported_type: Option<String>
}

