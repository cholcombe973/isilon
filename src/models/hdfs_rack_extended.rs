#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HdfsRackExtended {
    /// Array of IP ranges. Clients from one of these IP ranges are served by corresponding nodes from ip_pools array.
    #[serde(rename = "client_ip_ranges")]
    pub client_ip_ranges: Option<Vec<String>>,
    /// Array of IP pool names to use for serving clients from client_ip_ranges.
    #[serde(rename = "ip_pools")]
    pub ip_pools: Option<Vec<String>>,
    /// Name of the rack
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The ID of the rack.
    #[serde(rename = "id")]
    pub id: Option<String>,
}
