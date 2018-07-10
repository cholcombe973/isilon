

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareTapeNameParams {
  /// The name of the device
  #[serde(rename = "name")]
  name: Option<String>,
  /// Set the device state to close
  #[serde(rename = "state")]
  state: Option<String>
}

