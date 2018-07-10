#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDrivesPurposelistError {
    /// The general meaning of the status code.
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Logical node number of the node reporting this error.
    #[serde(rename = "lnn")]
    pub lnn: Option<i32>,
    /// More detailed description of the error.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// HTTP Status code returned by this node.
    #[serde(rename = "status")]
    pub status: Option<i32>,
}
