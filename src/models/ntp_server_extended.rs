#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NtpServerExtended {
    /// Field ID.
    #[serde(rename = "id")]
    pub id: String,
    /// Key value from key_file that maps to this server.
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// NTP server name.
    #[serde(rename = "name")]
    pub name: String,
}
