#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpLogsNode {
    /// The page on this node's NDMP log file being returned.
    #[serde(rename = "current_page")]
    pub current_page: Option<String>,
    /// Error message, if the HTTP status returned from this node was not 200.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Node ID of the node reporting this information.
    #[serde(rename = "id")]
    pub id: Option<i32>,
    /// Logical node number of the node reporting this information.
    #[serde(rename = "lnn")]
    pub lnn: Option<i32>,
    /// The log file entries from the current page on this node.
    #[serde(rename = "logs")]
    pub logs: Option<String>,
    /// The highest page number in this node's NDMP log file.
    #[serde(rename = "max_page")]
    pub max_page: Option<i32>,
    /// Status of the HTTP response from this node if not 200.  If 200, this field does not appear.
    #[serde(rename = "status")]
    pub status: Option<i32>,
}
