#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterVersionNode {
    /// OneFS build string.
    #[serde(rename = "build")]
    pub build: String,
    /// Error message, if the HTTP status returned from this node was not 200.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Node ID of the node reporting this information.
    #[serde(rename = "id")]
    pub id: Option<i32>,
    /// Logical node number of the node reporting this information.
    #[serde(rename = "lnn")]
    pub lnn: Option<i32>,
    /// Kernel release number.
    #[serde(rename = "release")]
    pub release: String,
    /// OneFS build number.
    #[serde(rename = "revision")]
    pub revision: String,
    /// Status of the HTTP response from this node if not 200.  If 200, this field does not appear.
    #[serde(rename = "status")]
    pub status: Option<i32>,
    /// Kernel release type.
    #[serde(rename = "type")]
    pub _type: String,
    /// Kernel full version information.
    #[serde(rename = "version")]
    pub version: String,
}
