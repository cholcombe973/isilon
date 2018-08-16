#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticsNetloggerSettings {
    /// IP Addresses or host names of clients
    #[serde(rename = "clients")]
    pub clients: Option<String>,
    /// Count of capture files to keep, 0 is infinite.
    #[serde(rename = "count")]
    pub count: Option<i32>,
    /// Duration in minutes of each capture file
    #[serde(rename = "duration")]
    pub duration: Option<i32>,
    /// Network interfaces to capture on.
    #[serde(rename = "interfaces")]
    pub interfaces: Option<String>,
    /// List of nodes, or empty for all
    #[serde(rename = "nodelist")]
    pub nodelist: Option<String>,
    /// List of Integers of TCP or UDP ports
    #[serde(rename = "ports")]
    pub ports: Option<String>,
    /// which protocol(s) to gather on
    #[serde(rename = "protocols")]
    pub protocols: Option<String>,
    /// Amount of bytes per packet to capture
    #[serde(rename = "snaplength")]
    pub snaplength: Option<u64>,
}
