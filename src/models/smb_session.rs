#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbSession {
    /// Number of seconds since session start.
    #[serde(rename = "active_time")]
    pub active_time: i32,
    /// Client type.
    #[serde(rename = "client_type")]
    pub client_type: String,
    /// Client internet address.
    #[serde(rename = "computer")]
    pub computer: String,
    /// True if session is encrypted.
    #[serde(rename = "encryption")]
    pub encryption: bool,
    /// True for guest logins.
    #[serde(rename = "guest_login")]
    pub guest_login: bool,
    /// The session ID.
    #[serde(rename = "id")]
    pub id: i32,
    /// Number of seconds since last client operation.
    #[serde(rename = "idle_time")]
    pub idle_time: i32,
    /// Number of files open by client.
    #[serde(rename = "openfiles")]
    pub openfiles: i32,
    /// Local user name.
    #[serde(rename = "user")]
    pub user: String,
}
