#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetsSubnetPoolsPool {
    /// Name of a valid access zone to map IP address pool to the zone.
    #[serde(rename = "access_zone")]
    pub access_zone: String,
    /// IP address format.
    #[serde(rename = "addr_family")]
    pub addr_family: String,
    /// OneFS supports the following NIC aggregation modes.
    #[serde(rename = "aggregation_mode")]
    pub aggregation_mode: String,
    /// Specifies how IP address allocation is done among pool members.
    #[serde(rename = "alloc_method")]
    pub alloc_method: String,
    /// A description of the pool.
    #[serde(rename = "description")]
    pub description: String,
    /// Name of the groupnet this pool belongs to.
    #[serde(rename = "groupnet")]
    pub groupnet: String,
    /// Unique Pool ID.
    #[serde(rename = "id")]
    pub id: String,
    /// List of interface members in this pool.
    #[serde(rename = "ifaces")]
    pub ifaces: Vec<::models::SubnetsSubnetPoolIface>,
    /// The name of the pool. It must be unique throughout the given subnet.It's a required field with POST method.
    #[serde(rename = "name")]
    pub name: String,
    /// List of IP address ranges in this pool.
    #[serde(rename = "ranges")]
    pub ranges: Vec<::models::SubnetsSubnetPoolRange>,
    /// Rebalance policy..
    #[serde(rename = "rebalance_policy")]
    pub rebalance_policy: String,
    /// Names of the rules in this pool.
    #[serde(rename = "rules")]
    pub rules: Vec<String>,
    /// Time delay in seconds before a node which has been                 automatically unsuspended becomes usable in SmartConnect                responses for pool zones.
    #[serde(rename = "sc_auto_unsuspend_delay")]
    pub sc_auto_unsuspend_delay: i32,
    /// SmartConnect client connection balancing policy.
    #[serde(rename = "sc_connect_policy")]
    pub sc_connect_policy: String,
    /// SmartConnect zone name for the pool.
    #[serde(rename = "sc_dns_zone")]
    pub sc_dns_zone: String,
    /// List of SmartConnect zone aliases (DNS names) to the pool.
    #[serde(rename = "sc_dns_zone_aliases")]
    pub sc_dns_zone_aliases: Vec<String>,
    /// SmartConnect IP failover policy.
    #[serde(rename = "sc_failover_policy")]
    pub sc_failover_policy: String,
    /// Name of SmartConnect service subnet for this pool.
    #[serde(rename = "sc_subnet")]
    pub sc_subnet: String,
    /// List of LNNs showing currently suspended nodes in SmartConnect.
    #[serde(rename = "sc_suspended_nodes")]
    pub sc_suspended_nodes: Vec<i32>,
    /// Time to live value for SmartConnect DNS query responses in seconds.
    #[serde(rename = "sc_ttl")]
    pub sc_ttl: i32,
    /// List of interface members in this pool.
    #[serde(rename = "static_routes")]
    pub static_routes: Vec<::models::SubnetsSubnetPoolStaticRoute>,
    /// The name of the subnet.
    #[serde(rename = "subnet")]
    pub subnet: String,
}
