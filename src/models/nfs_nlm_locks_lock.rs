#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsNlmLocksLock {
    /// Specifies the client host name and IP address.
    #[serde(rename = "client")]
    pub client: Option<String>,
    /// Specifies the client ID.
    #[serde(rename = "client_id")]
    pub client_id: Option<String>,
    /// Specifies the UNIX EPoch time that the lock was created.
    #[serde(rename = "created")]
    pub created: Option<i32>,
    /// Specifies the system-assigned ID given to the lock. This value is returned when the lock is created with the POST method.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Specifies the LIN in /ifs that is locked.
    #[serde(rename = "lin")]
    pub lin: Option<String>,
    /// Specifies the lock type.
    #[serde(rename = "lock_type")]
    pub lock_type: Option<String>,
    /// Specifies the path under /ifs that is locked.
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// Specifies the byte range within the locked file.
    #[serde(rename = "range")]
    pub range: Option<Vec<i32>>,
}
