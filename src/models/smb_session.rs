

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbSession {
  /// Number of seconds since session start.
  #[serde(rename = "active_time")]
  active_time: i32,
  /// Client type.
  #[serde(rename = "client_type")]
  client_type: String,
  /// Client internet address.
  #[serde(rename = "computer")]
  computer: String,
  /// True if session is encrypted.
  #[serde(rename = "encryption")]
  encryption: bool,
  /// True for guest logins.
  #[serde(rename = "guest_login")]
  guest_login: bool,
  /// The session ID.
  #[serde(rename = "id")]
  id: i32,
  /// Number of seconds since last client operation.
  #[serde(rename = "idle_time")]
  idle_time: i32,
  /// Number of files open by client.
  #[serde(rename = "openfiles")]
  openfiles: i32,
  /// Local user name.
  #[serde(rename = "user")]
  user: String
}

