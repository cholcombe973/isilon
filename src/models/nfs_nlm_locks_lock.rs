

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsNlmLocksLock {
  /// Specifies the client host name and IP address.
  #[serde(rename = "client")]
  client: Option<String>,
  /// Specifies the client ID.
  #[serde(rename = "client_id")]
  client_id: Option<String>,
  /// Specifies the UNIX EPoch time that the lock was created.
  #[serde(rename = "created")]
  created: Option<i32>,
  /// Specifies the system-assigned ID given to the lock. This value is returned when the lock is created with the POST method.
  #[serde(rename = "id")]
  id: Option<String>,
  /// Specifies the LIN in /ifs that is locked.
  #[serde(rename = "lin")]
  lin: Option<String>,
  /// Specifies the lock type.
  #[serde(rename = "lock_type")]
  lock_type: Option<String>,
  /// Specifies the path under /ifs that is locked.
  #[serde(rename = "path")]
  path: Option<String>,
  /// Specifies the byte range within the locked file.
  #[serde(rename = "range")]
  range: Option<Vec<i32>>
}

