

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareFcport {
  /// The unique display id
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "rate")]
  rate: Option<String>,
  /// State of the port
  #[serde(rename = "state")]
  state: Option<String>,
  #[serde(rename = "topology")]
  topology: Option<String>,
  /// World wide node name of the port
  #[serde(rename = "wwnn")]
  wwnn: Option<String>,
  /// World wide port name of the port
  #[serde(rename = "wwpn")]
  wwpn: Option<String>
}

