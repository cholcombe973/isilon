

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodesOnefsVersion {
  #[serde(rename = "bugfix")]
  bugfix: Option<i32>,
  #[serde(rename = "maintenance")]
  maintenance: Option<i32>,
  #[serde(rename = "major")]
  major: Option<i32>,
  #[serde(rename = "minor")]
  minor: Option<i32>,
  /// hex representation of the OneFS version integer.
  #[serde(rename = "version")]
  version: Option<String>
}

