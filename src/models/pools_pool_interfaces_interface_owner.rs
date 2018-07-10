

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolsPoolInterfacesInterfaceOwner {
  #[serde(rename = "groupnet")]
  groupnet: Option<String>,
  #[serde(rename = "pool")]
  pool: Option<String>,
  #[serde(rename = "subnet")]
  subnet: Option<String>
}

