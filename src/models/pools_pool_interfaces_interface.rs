

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolsPoolInterfacesInterface {
  /// Unique interface ID.
  #[serde(rename = "id")]
  id: String,
  /// List of IP addresses
  #[serde(rename = "ip_addrs")]
  ip_addrs: Vec<String>,
  /// Logical Node Number
  #[serde(rename = "lnn")]
  lnn: i32,
  /// The name of the interface.
  #[serde(rename = "name")]
  name: String,
  /// NIC name
  #[serde(rename = "nic_name")]
  nic_name: String,
  /// List of owners (membership)
  #[serde(rename = "owners")]
  owners: Vec<::models::PoolsPoolInterfacesInterfaceOwner>,
  /// Status of the interface
  #[serde(rename = "status")]
  status: String,
  #[serde(rename = "type")]
  _type: String
}

