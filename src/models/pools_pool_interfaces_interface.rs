#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolsPoolInterfacesInterface {
    /// Unique interface ID.
    #[serde(rename = "id")]
    pub id: String,
    /// List of IP addresses
    #[serde(rename = "ip_addrs")]
    pub ip_addrs: Vec<String>,
    /// Logical Node Number
    #[serde(rename = "lnn")]
    pub lnn: i32,
    /// The name of the interface.
    #[serde(rename = "name")]
    pub name: String,
    /// NIC name
    #[serde(rename = "nic_name")]
    pub nic_name: String,
    /// List of owners (membership)
    #[serde(rename = "owners")]
    pub owners: Vec <crate::models::PoolsPoolInterfacesInterfaceOwner>,
    /// Status of the interface
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "type")]
    pub _type: String,
}
