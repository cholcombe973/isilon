/// NtpServer : NTP server.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NtpServer {
    /// Key value from key_file that maps to this server.
    #[serde(rename = "key")]
    pub key: String,
}
