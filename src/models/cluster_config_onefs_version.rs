

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterConfigOnefsVersion {
  /// OneFS build string.
  #[serde(rename = "build")]
  build: String,
  /// Cluster copyright information.
  #[serde(rename = "copyright")]
  copyright: String,
  /// Timestamp of release date.
  #[serde(rename = "reldate")]
  reldate: i32,
  /// Kernel release number.
  #[serde(rename = "release")]
  release: String,
  /// OneFS build number.
  #[serde(rename = "revision")]
  revision: String,
  /// Kernel release type.
  #[serde(rename = "type")]
  _type: String,
  /// Kernel full version information.
  #[serde(rename = "version")]
  version: String
}

