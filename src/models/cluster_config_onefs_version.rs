#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterConfigOnefsVersion {
    /// OneFS build string.
    #[serde(rename = "build")]
    pub build: String,
    /// Cluster copyright information.
    #[serde(rename = "copyright")]
    pub copyright: String,
    /// Timestamp of release date.
    #[serde(rename = "reldate")]
    pub reldate: i32,
    /// Kernel release number.
    #[serde(rename = "release")]
    pub release: String,
    /// OneFS build number.
    #[serde(rename = "revision")]
    pub revision: String,
    /// Kernel release type.
    #[serde(rename = "type")]
    pub _type: String,
    /// Kernel full version information.
    #[serde(rename = "version")]
    pub version: String,
}
