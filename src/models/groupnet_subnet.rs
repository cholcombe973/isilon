#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupnetSubnet {
    /// A description of the subnet.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// List of Direct Server Return addresses.
    #[serde(rename = "dsr_addrs")]
    pub dsr_addrs: Option<Vec<String>>,
    /// Gateway IP address.
    #[serde(rename = "gateway")]
    pub gateway: Option<String>,
    /// Gateway priority.
    #[serde(rename = "gateway_priority")]
    pub gateway_priority: Option<i32>,
    /// MTU of the subnet.
    #[serde(rename = "mtu")]
    pub mtu: Option<i32>,
    /// The name of the subnet.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Subnet Prefix Length.
    #[serde(rename = "prefixlen")]
    pub prefixlen: Option<i32>,
    /// The address that SmartConnect listens for DNS requests.
    #[serde(rename = "sc_service_addr")]
    pub sc_service_addr: Option<String>,
    /// Domain Name corresponding to the SmartConnect Service Address.
    #[serde(rename = "sc_service_name")]
    pub sc_service_name: Option<String>,
    /// VLAN tagging enabled or disabled.
    #[serde(rename = "vlan_enabled")]
    pub vlan_enabled: Option<bool>,
    /// VLAN ID for all interfaces in the subnet.
    #[serde(rename = "vlan_id")]
    pub vlan_id: Option<i32>,
}
