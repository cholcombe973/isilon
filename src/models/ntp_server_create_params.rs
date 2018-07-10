/// NtpServerCreateParams : NTP server.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NtpServerCreateParams {
    /// Key value from key_file that maps to this server.
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// NTP server name.
    #[serde(rename = "name")]
    pub name: String,
}
