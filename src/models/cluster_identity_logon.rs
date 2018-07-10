#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterIdentityLogon {
    /// The message of the day.
    #[serde(rename = "motd")]
    pub motd: String,
    /// The header to the message of the day.
    #[serde(rename = "motd_header")]
    pub motd_header: String,
}
