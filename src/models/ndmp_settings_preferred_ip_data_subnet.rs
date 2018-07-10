

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsPreferredIpDataSubnet {
  /// The name of a subnet.
  #[serde(rename = "subnet")]
  subnet: Option<String>
}

