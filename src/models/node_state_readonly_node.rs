#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStateReadonlyNode {
    /// The current read-only mode allowed status for the node.
    #[serde(rename = "allowed")]
    pub allowed: Option<bool>,
    /// The current read-only user mode status for the node. NOTE: If read-only mode is currently disallowed for this node, it will remain read/write until read-only mode is allowed again. This value only sets or clears any user-specified requests for read-only mode. If the node has been placed into read-only mode by the system, it will remain in read-only mode until the system conditions which triggered read-only mode have cleared.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Error message, if the HTTP status returned from this node was not 200.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Node ID of the node reporting this information.
    #[serde(rename = "id")]
    pub id: Option<i32>,
    /// Logical node number of the node reporting this information.
    #[serde(rename = "lnn")]
    pub lnn: Option<i32>,
    /// The current read-only mode status for the node.
    #[serde(rename = "mode")]
    pub mode: Option<bool>,
    /// The current read-only mode status description for the node.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// The read-only state values are valid (False = Error).
    #[serde(rename = "valid")]
    pub valid: Option<bool>,
    /// The current read-only value (enumerated bitfield) for the node.
    #[serde(rename = "value")]
    pub value: Option<i32>,
}
