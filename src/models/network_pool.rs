#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkPool {
    /// Name of a valid access zone to map IP address pool to the zone.
    #[serde(rename = "access_zone")]
    pub access_zone: Option<String>,
    /// IP address format.
    #[serde(rename = "addr_family")]
    pub addr_family: Option<String>,
    /// OneFS supports the following NIC aggregation modes.
    #[serde(rename = "aggregation_mode")]
    pub aggregation_mode: Option<String>,
    /// Specifies how IP address allocation is done among pool members.
    #[serde(rename = "alloc_method")]
    pub alloc_method: Option<String>,
    /// A description of the pool.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Name of the groupnet this pool belongs to.
    #[serde(rename = "groupnet")]
    pub groupnet: Option<String>,
    /// Unique Pool ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// List of interface members in this pool.
    #[serde(rename = "ifaces")]
    pub ifaces: Option<Vec<::models::SubnetsSubnetPoolIface>>,
    /// The name of the pool. It must be unique throughout the given subnet.It's a required field with POST method.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// List of IP address ranges in this pool.
    #[serde(rename = "ranges")]
    pub ranges: Option<Vec<::models::SubnetsSubnetPoolRange>>,
    /// Rebalance policy..
    #[serde(rename = "rebalance_policy")]
    pub rebalance_policy: Option<String>,
    /// Names of the rules in this pool.
    #[serde(rename = "rules")]
    pub rules: Option<Vec<String>>,
    /// Time delay in seconds before a node which has been                 automatically unsuspended becomes usable in SmartConnect                responses for pool zones.
    #[serde(rename = "sc_auto_unsuspend_delay")]
    pub sc_auto_unsuspend_delay: Option<i32>,
    /// SmartConnect client connection balancing policy.
    #[serde(rename = "sc_connect_policy")]
    pub sc_connect_policy: Option<String>,
    /// SmartConnect zone name for the pool.
    #[serde(rename = "sc_dns_zone")]
    pub sc_dns_zone: Option<String>,
    /// List of SmartConnect zone aliases (DNS names) to the pool.
    #[serde(rename = "sc_dns_zone_aliases")]
    pub sc_dns_zone_aliases: Option<Vec<String>>,
    /// SmartConnect IP failover policy.
    #[serde(rename = "sc_failover_policy")]
    pub sc_failover_policy: Option<String>,
    /// Name of SmartConnect service subnet for this pool.
    #[serde(rename = "sc_subnet")]
    pub sc_subnet: Option<String>,
    /// List of LNNs showing currently suspended nodes in SmartConnect.
    #[serde(rename = "sc_suspended_nodes")]
    pub sc_suspended_nodes: Option<Vec<i32>>,
    /// Time to live value for SmartConnect DNS query responses in seconds.
    #[serde(rename = "sc_ttl")]
    pub sc_ttl: Option<i32>,
    /// List of interface members in this pool.
    #[serde(rename = "static_routes")]
    pub static_routes: Option<Vec<::models::SubnetsSubnetPoolStaticRoute>>,
    /// The name of the subnet.
    #[serde(rename = "subnet")]
    pub subnet: Option<String>,
}
