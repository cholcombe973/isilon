

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterVersionNode {
  /// OneFS build string.
  #[serde(rename = "build")]
  build: String,
  /// Error message, if the HTTP status returned from this node was not 200.
  #[serde(rename = "error")]
  error: Option<String>,
  /// Node ID of the node reporting this information.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// Logical node number of the node reporting this information.
  #[serde(rename = "lnn")]
  lnn: Option<i32>,
  /// Kernel release number.
  #[serde(rename = "release")]
  release: String,
  /// OneFS build number.
  #[serde(rename = "revision")]
  revision: String,
  /// Status of the HTTP response from this node if not 200.  If 200, this field does not appear.
  #[serde(rename = "status")]
  status: Option<i32>,
  /// Kernel release type.
  #[serde(rename = "type")]
  _type: String,
  /// Kernel full version information.
  #[serde(rename = "version")]
  version: String
}

