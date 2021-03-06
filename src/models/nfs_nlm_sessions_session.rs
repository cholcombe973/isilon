#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsNlmSessionsSession {
    #[serde(rename = "delegates")]
    pub delegates: Option<Vec<i32>>,
    /// The sort of host that this entry represents
    #[serde(rename = "host_type")]
    pub host_type: Option<String>,
    /// The host being monitored
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    /// Whether or not the client is actively being monitored
    #[serde(rename = "is_active")]
    pub is_active: Option<bool>,
    /// Unix time in seconds that the client was last modified (monitored or unmonitored)
    #[serde(rename = "last_modified")]
    pub last_modified: Option<i32>,
    /// An IP address for which NSM has client records
    #[serde(rename = "node_ip")]
    pub node_ip: Option<String>,
    /// Number of times we will attempt to notify this client before giving up
    #[serde(rename = "notify_attempts_remaining")]
    pub notify_attempts_remaining: Option<i32>,
    /// Last error recieved attempting to notify this client
    #[serde(rename = "notify_error")]
    pub notify_error: Option<String>,
    /// Unix time in seconds when we last attempted to notify this clients
    #[serde(rename = "notify_last_attempt")]
    pub notify_last_attempt: Option<i32>,
}
